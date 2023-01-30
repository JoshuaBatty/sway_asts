


TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 80,
            end: 86,
            as_str(): "Result",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(31632),
        E: TypeId(31633),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 99,
                    end: 101,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31632,
            ),
            initial_type_id: TypeId(
                31634,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0f64c4390,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                ),
                start: 103,
                end: 104,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb0f64c4390,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                ),
                start: 99,
                end: 104,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 110,
                    end: 113,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31633,
            ),
            initial_type_id: TypeId(
                31635,
            ),
            type_span: Span {
                src (ptr): 0x00007fb0f64c4390,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                ),
                start: 115,
                end: 116,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb0f64c4390,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                ),
                start: 110,
                end: 116,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb0f64c4390,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
        ),
        start: 75,
        end: 119,
        as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
    },
    visibility: Private,
}
TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 127,
            end: 129,
            as_str(): "B1",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: StructExpression {
            struct_name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb1150d1f00,
                    path: Some(
                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                    ),
                    start: 188,
                    end: 195,
                    as_str(): "Address",
                },
                is_raw_ident: false,
            },
            fields: [
                TyStructExpressionField {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 146,
                            end: 151,
                            as_str(): "value",
                        },
                        is_raw_ident: false,
                    },
                    value: TyExpression {
                        expression: Literal(
                            B256(
                                [
                                    1,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    16,
                                ],
                            ),
                        ),
                        return_type: TypeId(
                            59,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 153,
                            end: 219,
                            as_str(): "0x0100000000000000000000000000000000000000000000000000000000000010",
                        },
                    },
                },
            ],
            span: Span {
                src (ptr): 0x00007fb0f64c4390,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                ),
                start: 132,
                end: 139,
                as_str(): "Address",
            },
        },
        return_type: TypeId(
            9112,
        ),
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 132,
            end: 221,
            as_str(): "Address {\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n}",
        },
    },
    visibility: Private,
    return_type: TypeId(
        9112,
    ),
    is_configurable: false,
    attributes: {},
    type_ascription_span: None,
    span: Span {
        src (ptr): 0x00007fb0f64c4390,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
        ),
        start: 121,
        end: 222,
        as_str(): "const B1 = Address {\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n};",
    },
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb0f64c4390,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
            ),
            start: 227,
            end: 231,
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
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 251,
                                    end: 252,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1150383a0,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                ),
                                                start: 1808,
                                                end: 1814,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(21),
                                            E: TypeId(21),
                                        ],
                                        attributes: {
                                            DocComment: [
                                                Attribute {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "doc-comment",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1150383a0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                            ),
                                                            start: 1710,
                                                            end: 1783,
                                                            as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    args: [
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1713,
                                                                end: 1783,
                                                                as_str(): " `Result` is a type that represents either success ([`Ok`]) or failure",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1710,
                                                        end: 1783,
                                                        as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                    },
                                                },
                                                Attribute {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "doc-comment",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1150383a0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                            ),
                                                            start: 1784,
                                                            end: 1798,
                                                            as_str(): "/// ([`Err`]).",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    args: [
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1787,
                                                                end: 1798,
                                                                as_str(): " ([`Err`]).",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1784,
                                                        end: 1798,
                                                        as_str(): "/// ([`Err`]).",
                                                    },
                                                },
                                            ],
                                        },
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1862,
                                                        end: 1864,
                                                        as_str(): "Ok",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7487,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1866,
                                                    end: 1867,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1862,
                                                    end: 1867,
                                                    as_str(): "Ok: T",
                                                },
                                                attributes: {
                                                    DocComment: [
                                                        Attribute {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "doc-comment",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1150383a0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                    ),
                                                                    start: 1827,
                                                                    end: 1857,
                                                                    as_str(): "/// Contains the success value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            args: [
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1150383a0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                        ),
                                                                        start: 1830,
                                                                        end: 1857,
                                                                        as_str(): " Contains the success value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1827,
                                                                end: 1857,
                                                                as_str(): "/// Contains the success value",
                                                            },
                                                        },
                                                    ],
                                                },
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1906,
                                                        end: 1909,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7488,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1911,
                                                    end: 1912,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1906,
                                                    end: 1912,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {
                                                    DocComment: [
                                                        Attribute {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "doc-comment",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1150383a0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                    ),
                                                                    start: 1873,
                                                                    end: 1901,
                                                                    as_str(): "/// Contains the error value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            args: [
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1150383a0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                        ),
                                                                        start: 1876,
                                                                        end: 1901,
                                                                        as_str(): " Contains the error value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1873,
                                                                end: 1901,
                                                                as_str(): "/// Contains the error value",
                                                            },
                                                        },
                                                    ],
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb1150383a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                            ),
                                            start: 1799,
                                            end: 1915,
                                            as_str(): "pub enum Result<T, E> {\n    /// Contains the success value\n    Ok: T,\n    /// Contains the error value\n    Err: E,\n}",
                                        },
                                        visibility: Public,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1150383a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                            ),
                                            start: 1862,
                                            end: 1864,
                                            as_str(): "Ok",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    100,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 278,
                                                end: 281,
                                                as_str(): "100",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 255,
                                        end: 261,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 263,
                                        end: 265,
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
                                                    7259,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 268,
                                                    end: 271,
                                                    as_str(): "u64",
                                                },
                                            },
                                            TypeArgument {
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7260,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 273,
                                                    end: 276,
                                                    as_str(): "u64",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 255,
                                            end: 277,
                                            as_str(): "Result::Ok::<u64, u64>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31668,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 263,
                                    end: 265,
                                    as_str(): "Ok",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31668,
                            ),
                            type_ascription: TypeId(
                                31640,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 247,
                    end: 283,
                    as_str(): "let a = Result::Ok::<u64, u64>(100);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 292,
                                    end: 293,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 319,
                                                            end: 320,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 319,
                                                            end: 320,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 319,
                                                        end: 320,
                                                        as_str(): "a",
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
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 251,
                                                                            end: 252,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 319,
                                                                        end: 320,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31668,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 319,
                                                                    end: 320,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 319,
                                                            end: 320,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 319,
                                                            end: 320,
                                                            as_str(): "a",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13334,
                                                Span {
                                                    src (ptr): 0x00007fb1094e94d0,
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 319,
                                            end: 320,
                                            as_str(): "a",
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 314,
                                                                            end: 315,
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
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 251,
                                                                                            end: 252,
                                                                                            as_str(): "a",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 319,
                                                                                        end: 320,
                                                                                        as_str(): "a",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31668,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 319,
                                                                                    end: 320,
                                                                                    as_str(): "a",
                                                                                },
                                                                            },
                                                                            variant: TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1150383a0,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1862,
                                                                                        end: 1864,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    31671,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7487,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1866,
                                                                                    end: 1867,
                                                                                    as_str(): "T",
                                                                                },
                                                                                tag: 0,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1862,
                                                                                    end: 1867,
                                                                                    as_str(): "Ok: T",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1827,
                                                                                                    end: 1857,
                                                                                                    as_str(): "/// Contains the success value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1150383a0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1830,
                                                                                                        end: 1857,
                                                                                                        as_str(): " Contains the success value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1150383a0,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1827,
                                                                                                end: 1857,
                                                                                                as_str(): "/// Contains the success value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31671,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 303,
                                                                            end: 316,
                                                                            as_str(): "Result::Ok(y)",
                                                                        },
                                                                    },
                                                                    mutability: Immutable,
                                                                    return_type: TypeId(
                                                                        31671,
                                                                    ),
                                                                    type_ascription: TypeId(
                                                                        31671,
                                                                    ),
                                                                    type_ascription_span: None,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 314,
                                                            end: 315,
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
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 325,
                                                                                    end: 326,
                                                                                    as_str(): "+",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 325,
                                                                                    end: 326,
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 325,
                                                                                end: 326,
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
                                                                                    src (ptr): 0x00007fb1094e94d0,
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
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 314,
                                                                                            end: 315,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 323,
                                                                                        end: 324,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31671,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 323,
                                                                                    end: 324,
                                                                                    as_str(): "y",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1094e94d0,
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
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 327,
                                                                                    end: 329,
                                                                                    as_str(): "10",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        13333,
                                                                        Span {
                                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 325,
                                                                                end: 326,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 323,
                                                                    end: 329,
                                                                    as_str(): "y + 10",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 323,
                                                            end: 329,
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 321,
                                            end: 331,
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
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 339,
                                                                        end: 340,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 339,
                                                                end: 340,
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
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 337,
                                                end: 342,
                                                as_str(): "{ 1 }",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 321,
                                    end: 331,
                                    as_str(): "{ y + 10 }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31669,
                            ),
                            type_ascription: TypeId(
                                31669,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 288,
                    end: 343,
                    as_str(): "let b = if let Result::Ok(y) = a { y + 10 } else { 1 };",
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
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 348,
                                        end: 354,
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
                                            src (ptr): 0x00007fb10190a910,
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 357,
                                                            end: 359,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 357,
                                                            end: 359,
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
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 357,
                                                        end: 359,
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
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 292,
                                                                    end: 293,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 355,
                                                                end: 356,
                                                                as_str(): "b",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            31669,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 355,
                                                            end: 356,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                                110,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 360,
                                                            end: 363,
                                                            as_str(): "110",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13336,
                                                Span {
                                                    src (ptr): 0x00007fb1094e94d0,
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
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 357,
                                                        end: 359,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 355,
                                            end: 363,
                                            as_str(): "b == 110",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13337,
                                Span {
                                    src (ptr): 0x00007fb10190a910,
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
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 348,
                                        end: 354,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31688,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 348,
                            end: 364,
                            as_str(): "assert(b == 110)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 348,
                    end: 364,
                    as_str(): "assert(b == 110)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 375,
                                    end: 381,
                                    as_str(): "sender",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb0ff2e8bd0,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                ),
                                                start: 277,
                                                end: 285,
                                                as_str(): "Identity",
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
                                                        src (ptr): 0x00007fb0ff2e8bd0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                        ),
                                                        start: 292,
                                                        end: 299,
                                                        as_str(): "Address",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    9112,
                                                ),
                                                initial_type_id: TypeId(
                                                    9178,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb0ff2e8bd0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 301,
                                                    end: 308,
                                                    as_str(): "Address",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb0ff2e8bd0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 292,
                                                    end: 308,
                                                    as_str(): "Address: Address",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ff2e8bd0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                        ),
                                                        start: 314,
                                                        end: 324,
                                                        as_str(): "ContractId",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7861,
                                                ),
                                                initial_type_id: TypeId(
                                                    9179,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb0ff2e8bd0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 326,
                                                    end: 336,
                                                    as_str(): "ContractId",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb0ff2e8bd0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 314,
                                                    end: 336,
                                                    as_str(): "ContractId: ContractId",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0ff2e8bd0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                            ),
                                            start: 268,
                                            end: 339,
                                            as_str(): "pub enum Identity {\n    Address: Address,\n    ContractId: ContractId,\n}",
                                        },
                                        visibility: Public,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0ff2e8bd0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                            ),
                                            start: 292,
                                            end: 299,
                                            as_str(): "Address",
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
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 127,
                                                        end: 129,
                                                        as_str(): "B1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 402,
                                                    end: 404,
                                                    as_str(): "B1",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                9112,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 402,
                                                end: 404,
                                                as_str(): "B1",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 384,
                                        end: 392,
                                        as_str(): "Identity",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 394,
                                        end: 401,
                                        as_str(): "Address",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 384,
                                            end: 401,
                                            as_str(): "Identity::Address",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    9181,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 394,
                                    end: 401,
                                    as_str(): "Address",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                9181,
                            ),
                            type_ascription: TypeId(
                                31689,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 371,
                    end: 406,
                    as_str(): "let sender = Identity::Address(B1);",
                },
            },
            TyAstNode {
                content: Expression(
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
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 445,
                                                    end: 451,
                                                    as_str(): "sender",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 445,
                                                    end: 451,
                                                    as_str(): "sender",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 445,
                                                end: 451,
                                                as_str(): "sender",
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
                                                    src (ptr): 0x00007fb1094e94d0,
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
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 375,
                                                                    end: 381,
                                                                    as_str(): "sender",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 445,
                                                                end: 451,
                                                                as_str(): "sender",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            9181,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 445,
                                                            end: 451,
                                                            as_str(): "sender",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 445,
                                                    end: 451,
                                                    as_str(): "sender",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1094e94d0,
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
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 445,
                                                    end: 451,
                                                    as_str(): "sender",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13345,
                                        Span {
                                            src (ptr): 0x00007fb1094e94d0,
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
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 445,
                                    end: 451,
                                    as_str(): "sender",
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
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 436,
                                                                    end: 441,
                                                                    as_str(): "addr1",
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
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 375,
                                                                                    end: 381,
                                                                                    as_str(): "sender",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                ),
                                                                                start: 445,
                                                                                end: 451,
                                                                                as_str(): "sender",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            9181,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 445,
                                                                            end: 451,
                                                                            as_str(): "sender",
                                                                        },
                                                                    },
                                                                    variant: TyEnumVariant {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ff2e8bd0,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                                                ),
                                                                                start: 292,
                                                                                end: 299,
                                                                                as_str(): "Address",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_id: TypeId(
                                                                            9112,
                                                                        ),
                                                                        initial_type_id: TypeId(
                                                                            9178,
                                                                        ),
                                                                        type_span: Span {
                                                                            src (ptr): 0x00007fb0ff2e8bd0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                                            ),
                                                                            start: 301,
                                                                            end: 308,
                                                                            as_str(): "Address",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0ff2e8bd0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                                            ),
                                                                            start: 292,
                                                                            end: 308,
                                                                            as_str(): "Address: Address",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    9112,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 418,
                                                                    end: 442,
                                                                    as_str(): "Identity::Address(addr1)",
                                                                },
                                                            },
                                                            mutability: Immutable,
                                                            return_type: TypeId(
                                                                9112,
                                                            ),
                                                            type_ascription: TypeId(
                                                                9112,
                                                            ),
                                                            type_ascription_span: None,
                                                        },
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 436,
                                                    end: 441,
                                                    as_str(): "addr1",
                                                },
                                            },
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: CodeBlock(
                                                            TyCodeBlock {
                                                                contents: [
                                                                    TyAstNode {
                                                                        content: Declaration(
                                                                            VariableDeclaration(
                                                                                TyVariableDeclaration {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "__match_return_var_name_1",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 474,
                                                                                            as_str(): "sender",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    body: TyExpression {
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                    ),
                                                                                                    start: 375,
                                                                                                    end: 381,
                                                                                                    as_str(): "sender",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 468,
                                                                                                end: 474,
                                                                                                as_str(): "sender",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            9181,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 474,
                                                                                            as_str(): "sender",
                                                                                        },
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                    return_type: TypeId(
                                                                                        9181,
                                                                                    ),
                                                                                    type_ascription: TypeId(
                                                                                        31695,
                                                                                    ),
                                                                                    type_ascription_span: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 462,
                                                                            end: 646,
                                                                            as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
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
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 468,
                                                                                                            end: 474,
                                                                                                            as_str(): "sender",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 468,
                                                                                                            end: 474,
                                                                                                            as_str(): "sender",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "eq",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                        ),
                                                                                                        start: 468,
                                                                                                        end: 474,
                                                                                                        as_str(): "sender",
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
                                                                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                                                                                        name_override_opt: Some(
                                                                                                                            "__match_return_var_name_1",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 468,
                                                                                                                            end: 474,
                                                                                                                            as_str(): "sender",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 468,
                                                                                                                        end: 474,
                                                                                                                        as_str(): "sender",
                                                                                                                    },
                                                                                                                    mutability: Immutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    9181,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 468,
                                                                                                                    end: 474,
                                                                                                                    as_str(): "sender",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 468,
                                                                                                            end: 474,
                                                                                                            as_str(): "sender",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 468,
                                                                                                            end: 474,
                                                                                                            as_str(): "sender",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            function_decl_id: DeclId(
                                                                                                13344,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fb1094e94d0,
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
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 474,
                                                                                            as_str(): "sender",
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
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 507,
                                                                                                                            end: 512,
                                                                                                                            as_str(): "addr2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    body: TyExpression {
                                                                                                                        expression: UnsafeDowncast {
                                                                                                                            exp: TyExpression {
                                                                                                                                expression: VariableExpression {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: Some(
                                                                                                                                            "__match_return_var_name_1",
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 468,
                                                                                                                                            end: 474,
                                                                                                                                            as_str(): "sender",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 468,
                                                                                                                                        end: 474,
                                                                                                                                        as_str(): "sender",
                                                                                                                                    },
                                                                                                                                    mutability: Immutable,
                                                                                                                                },
                                                                                                                                return_type: TypeId(
                                                                                                                                    9181,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 468,
                                                                                                                                    end: 474,
                                                                                                                                    as_str(): "sender",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            variant: TyEnumVariant {
                                                                                                                                name: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb0ff2e8bd0,
                                                                                                                                        path: Some(
                                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                                                                                                        ),
                                                                                                                                        start: 292,
                                                                                                                                        end: 299,
                                                                                                                                        as_str(): "Address",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                                type_id: TypeId(
                                                                                                                                    9112,
                                                                                                                                ),
                                                                                                                                initial_type_id: TypeId(
                                                                                                                                    9178,
                                                                                                                                ),
                                                                                                                                type_span: Span {
                                                                                                                                    src (ptr): 0x00007fb0ff2e8bd0,
                                                                                                                                    path: Some(
                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                                                                                                    ),
                                                                                                                                    start: 301,
                                                                                                                                    end: 308,
                                                                                                                                    as_str(): "Address",
                                                                                                                                },
                                                                                                                                tag: 0,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0ff2e8bd0,
                                                                                                                                    path: Some(
                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                                                                                                    ),
                                                                                                                                    start: 292,
                                                                                                                                    end: 308,
                                                                                                                                    as_str(): "Address: Address",
                                                                                                                                },
                                                                                                                                attributes: {},
                                                                                                                            },
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            9112,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 489,
                                                                                                                            end: 513,
                                                                                                                            as_str(): "Identity::Address(addr2)",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    mutability: Immutable,
                                                                                                                    return_type: TypeId(
                                                                                                                        9112,
                                                                                                                    ),
                                                                                                                    type_ascription: TypeId(
                                                                                                                        9112,
                                                                                                                    ),
                                                                                                                    type_ascription_span: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 507,
                                                                                                            end: 512,
                                                                                                            as_str(): "addr2",
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
                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 535,
                                                                                                                                end: 541,
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
                                                                                                                                    src (ptr): 0x00007fb10190a910,
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
                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 548,
                                                                                                                                                    end: 550,
                                                                                                                                                    as_str(): "==",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: Some(
                                                                                                                                                    "ops",
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 548,
                                                                                                                                                    end: 550,
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
                                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 548,
                                                                                                                                                end: 550,
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
                                                                                                                                                    src (ptr): 0x00007fb1150d1f00,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 261,
                                                                                                                                                    end: 265,
                                                                                                                                                    as_str(): "self",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            TyExpression {
                                                                                                                                                expression: VariableExpression {
                                                                                                                                                    name: BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 436,
                                                                                                                                                            end: 441,
                                                                                                                                                            as_str(): "addr1",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 542,
                                                                                                                                                        end: 547,
                                                                                                                                                        as_str(): "addr1",
                                                                                                                                                    },
                                                                                                                                                    mutability: Immutable,
                                                                                                                                                },
                                                                                                                                                return_type: TypeId(
                                                                                                                                                    9112,
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 542,
                                                                                                                                                    end: 547,
                                                                                                                                                    as_str(): "addr1",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        (
                                                                                                                                            BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb1150d1f00,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 267,
                                                                                                                                                    end: 272,
                                                                                                                                                    as_str(): "other",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            TyExpression {
                                                                                                                                                expression: VariableExpression {
                                                                                                                                                    name: BaseIdent {
                                                                                                                                                        name_override_opt: None,
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 507,
                                                                                                                                                            end: 512,
                                                                                                                                                            as_str(): "addr2",
                                                                                                                                                        },
                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                    },
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 551,
                                                                                                                                                        end: 556,
                                                                                                                                                        as_str(): "addr2",
                                                                                                                                                    },
                                                                                                                                                    mutability: Immutable,
                                                                                                                                                },
                                                                                                                                                return_type: TypeId(
                                                                                                                                                    9112,
                                                                                                                                                ),
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 551,
                                                                                                                                                    end: 556,
                                                                                                                                                    as_str(): "addr2",
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                    ],
                                                                                                                                    function_decl_id: DeclId(
                                                                                                                                        13340,
                                                                                                                                        Span {
                                                                                                                                            src (ptr): 0x00007fb1150d1f00,
                                                                                                                                            path: Some(
                                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                                                                                                                                            ),
                                                                                                                                            start: 255,
                                                                                                                                            end: 329,
                                                                                                                                            as_str(): "fn eq(self, other: Self) -> bool {\n        self.value == other.value\n    }",
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                    self_state_idx: None,
                                                                                                                                    selector: None,
                                                                                                                                    type_binding: Some(
                                                                                                                                        TypeBinding {
                                                                                                                                            inner: (),
                                                                                                                                            type_arguments: [],
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 548,
                                                                                                                                                end: 550,
                                                                                                                                                as_str(): "==",
                                                                                                                                            },
                                                                                                                                        },
                                                                                                                                    ),
                                                                                                                                },
                                                                                                                                return_type: TypeId(
                                                                                                                                    71,
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 542,
                                                                                                                                    end: 556,
                                                                                                                                    as_str(): "addr1 == addr2",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ],
                                                                                                                    function_decl_id: DeclId(
                                                                                                                        13341,
                                                                                                                        Span {
                                                                                                                            src (ptr): 0x00007fb10190a910,
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
                                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 535,
                                                                                                                                end: 541,
                                                                                                                                as_str(): "assert",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ),
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    31703,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 535,
                                                                                                                    end: 557,
                                                                                                                    as_str(): "assert(addr1 == addr2)",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                            ),
                                                                                                            start: 535,
                                                                                                            end: 557,
                                                                                                            as_str(): "assert(addr1 == addr2)",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            31705,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 517,
                                                                                            end: 572,
                                                                                            as_str(): "{\n                assert(addr1 == addr2);\n            }",
                                                                                        },
                                                                                    },
                                                                                    else: Some(
                                                                                        TyExpression {
                                                                                            expression: CodeBlock(
                                                                                                TyCodeBlock {
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
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 608,
                                                                                                                                    end: 614,
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
                                                                                                                                        src (ptr): 0x00007fb10190a910,
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
                                                                                                                                    expression: Literal(
                                                                                                                                        Boolean(
                                                                                                                                            false,
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                    return_type: TypeId(
                                                                                                                                        71,
                                                                                                                                    ),
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 615,
                                                                                                                                        end: 620,
                                                                                                                                        as_str(): "false",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ],
                                                                                                                        function_decl_id: DeclId(
                                                                                                                            13343,
                                                                                                                            Span {
                                                                                                                                src (ptr): 0x00007fb10190a910,
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
                                                                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 608,
                                                                                                                                    end: 614,
                                                                                                                                    as_str(): "assert",
                                                                                                                                },
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    return_type: TypeId(
                                                                                                                        31711,
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 608,
                                                                                                                        end: 621,
                                                                                                                        as_str(): "assert(false)",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                                ),
                                                                                                                start: 608,
                                                                                                                end: 621,
                                                                                                                as_str(): "assert(false)",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            return_type: TypeId(
                                                                                                31713,
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0f64c4390,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                                ),
                                                                                                start: 590,
                                                                                                end: 636,
                                                                                                as_str(): "{\n                assert(false);\n            }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31714,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 517,
                                                                                    end: 572,
                                                                                    as_str(): "{\n                assert(addr1 == addr2);\n            }",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 462,
                                                                            end: 646,
                                                                            as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            31715,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 462,
                                                            end: 646,
                                                            as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 462,
                                                    end: 646,
                                                    as_str(): "match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    31716,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 452,
                                    end: 652,
                                    as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [],
                                        },
                                    ),
                                    return_type: TypeId(
                                        31721,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 452,
                                        end: 652,
                                        as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31722,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 452,
                            end: 652,
                            as_str(): "{\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 411,
                    end: 652,
                    as_str(): "if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    }",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 663,
                                    end: 664,
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
                                                src (ptr): 0x00007fb1150383a0,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                ),
                                                start: 1808,
                                                end: 1814,
                                                as_str(): "Result",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(21),
                                            E: TypeId(21),
                                        ],
                                        attributes: {
                                            DocComment: [
                                                Attribute {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "doc-comment",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1150383a0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                            ),
                                                            start: 1710,
                                                            end: 1783,
                                                            as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    args: [
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1713,
                                                                end: 1783,
                                                                as_str(): " `Result` is a type that represents either success ([`Ok`]) or failure",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1710,
                                                        end: 1783,
                                                        as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                    },
                                                },
                                                Attribute {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "doc-comment",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1150383a0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                            ),
                                                            start: 1784,
                                                            end: 1798,
                                                            as_str(): "/// ([`Err`]).",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    args: [
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1787,
                                                                end: 1798,
                                                                as_str(): " ([`Err`]).",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1784,
                                                        end: 1798,
                                                        as_str(): "/// ([`Err`]).",
                                                    },
                                                },
                                            ],
                                        },
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1862,
                                                        end: 1864,
                                                        as_str(): "Ok",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7487,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1866,
                                                    end: 1867,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1862,
                                                    end: 1867,
                                                    as_str(): "Ok: T",
                                                },
                                                attributes: {
                                                    DocComment: [
                                                        Attribute {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "doc-comment",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1150383a0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                    ),
                                                                    start: 1827,
                                                                    end: 1857,
                                                                    as_str(): "/// Contains the success value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            args: [
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1150383a0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                        ),
                                                                        start: 1830,
                                                                        end: 1857,
                                                                        as_str(): " Contains the success value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1827,
                                                                end: 1857,
                                                                as_str(): "/// Contains the success value",
                                                            },
                                                        },
                                                    ],
                                                },
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1150383a0,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                        ),
                                                        start: 1906,
                                                        end: 1909,
                                                        as_str(): "Err",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7488,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1911,
                                                    end: 1912,
                                                    as_str(): "E",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb1150383a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                    ),
                                                    start: 1906,
                                                    end: 1912,
                                                    as_str(): "Err: E",
                                                },
                                                attributes: {
                                                    DocComment: [
                                                        Attribute {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "doc-comment",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1150383a0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                    ),
                                                                    start: 1873,
                                                                    end: 1901,
                                                                    as_str(): "/// Contains the error value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            args: [
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1150383a0,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                        ),
                                                                        start: 1876,
                                                                        end: 1901,
                                                                        as_str(): " Contains the error value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1150383a0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1873,
                                                                end: 1901,
                                                                as_str(): "/// Contains the error value",
                                                            },
                                                        },
                                                    ],
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb1150383a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                            ),
                                            start: 1799,
                                            end: 1915,
                                            as_str(): "pub enum Result<T, E> {\n    /// Contains the success value\n    Ok: T,\n    /// Contains the error value\n    Err: E,\n}",
                                        },
                                        visibility: Public,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1150383a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                            ),
                                            start: 1862,
                                            end: 1864,
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
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 708,
                                                end: 712,
                                                as_str(): "5u64",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 685,
                                        end: 691,
                                        as_str(): "Result",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 693,
                                        end: 695,
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
                                                    7259,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 698,
                                                    end: 701,
                                                    as_str(): "u64",
                                                },
                                            },
                                            TypeArgument {
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    7260,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 703,
                                                    end: 706,
                                                    as_str(): "u64",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 685,
                                            end: 707,
                                            as_str(): "Result::Ok::<u64, u64>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31778,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 693,
                                    end: 695,
                                    as_str(): "Ok",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31778,
                            ),
                            type_ascription: TypeId(
                                31724,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 666,
                                    end: 682,
                                    as_str(): "Result<u64, u64>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 659,
                    end: 714,
                    as_str(): "let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 724,
                                    end: 732,
                                    as_str(): "result_1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 759,
                                                            as_str(): "x",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 759,
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
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 758,
                                                        end: 759,
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
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 663,
                                                                            end: 664,
                                                                            as_str(): "x",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 759,
                                                                        as_str(): "x",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31778,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 758,
                                                                    end: 759,
                                                                    as_str(): "x",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 759,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 758,
                                                            end: 759,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13427,
                                                Span {
                                                    src (ptr): 0x00007fb1094e94d0,
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 758,
                                            end: 759,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 753,
                                                                            end: 754,
                                                                            as_str(): "x",
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
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 663,
                                                                                            end: 664,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 758,
                                                                                        end: 759,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31778,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 758,
                                                                                    end: 759,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                            variant: TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1150383a0,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1862,
                                                                                        end: 1864,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    31781,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7487,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1866,
                                                                                    end: 1867,
                                                                                    as_str(): "T",
                                                                                },
                                                                                tag: 0,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1862,
                                                                                    end: 1867,
                                                                                    as_str(): "Ok: T",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1827,
                                                                                                    end: 1857,
                                                                                                    as_str(): "/// Contains the success value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1150383a0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1830,
                                                                                                        end: 1857,
                                                                                                        as_str(): " Contains the success value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1150383a0,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1827,
                                                                                                end: 1857,
                                                                                                as_str(): "/// Contains the success value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31781,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 742,
                                                                            end: 755,
                                                                            as_str(): "Result::Ok(x)",
                                                                        },
                                                                    },
                                                                    mutability: Immutable,
                                                                    return_type: TypeId(
                                                                        31781,
                                                                    ),
                                                                    type_ascription: TypeId(
                                                                        31781,
                                                                    ),
                                                                    type_ascription_span: None,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 753,
                                                            end: 754,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                    TyAstNode {
                                                        content: ImplicitReturnExpression(
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        100,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 770,
                                                                    end: 773,
                                                                    as_str(): "100",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 770,
                                                            end: 773,
                                                            as_str(): "100",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 760,
                                            end: 779,
                                            as_str(): "{\n        100\n    }",
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
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 795,
                                                                        end: 796,
                                                                        as_str(): "1",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 795,
                                                                end: 796,
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
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 785,
                                                end: 802,
                                                as_str(): "{\n        1\n    }",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 760,
                                    end: 779,
                                    as_str(): "{\n        100\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31779,
                            ),
                            type_ascription: TypeId(
                                31779,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 720,
                    end: 803,
                    as_str(): "let result_1 = if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 812,
                                    end: 820,
                                    as_str(): "result_2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 847,
                                                            end: 848,
                                                            as_str(): "x",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 847,
                                                            end: 848,
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
                                                        src (ptr): 0x00007fb0f64c4390,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                        ),
                                                        start: 847,
                                                        end: 848,
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
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 663,
                                                                            end: 664,
                                                                            as_str(): "x",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 847,
                                                                        end: 848,
                                                                        as_str(): "x",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31778,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 847,
                                                                    end: 848,
                                                                    as_str(): "x",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 847,
                                                            end: 848,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1094e94d0,
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
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 847,
                                                            end: 848,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13428,
                                                Span {
                                                    src (ptr): 0x00007fb1094e94d0,
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 847,
                                            end: 848,
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
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 842,
                                                                            end: 843,
                                                                            as_str(): "x",
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
                                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                            ),
                                                                                            start: 663,
                                                                                            end: 664,
                                                                                            as_str(): "x",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                        ),
                                                                                        start: 847,
                                                                                        end: 848,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31778,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                                    ),
                                                                                    start: 847,
                                                                                    end: 848,
                                                                                    as_str(): "x",
                                                                                },
                                                                            },
                                                                            variant: TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1150383a0,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1906,
                                                                                        end: 1909,
                                                                                        as_str(): "Err",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    31794,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7488,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1911,
                                                                                    end: 1912,
                                                                                    as_str(): "E",
                                                                                },
                                                                                tag: 1,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1906,
                                                                                    end: 1912,
                                                                                    as_str(): "Err: E",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1150383a0,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1873,
                                                                                                    end: 1901,
                                                                                                    as_str(): "/// Contains the error value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1150383a0,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1876,
                                                                                                        end: 1901,
                                                                                                        as_str(): " Contains the error value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1150383a0,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1873,
                                                                                                end: 1901,
                                                                                                as_str(): "/// Contains the error value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31794,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0f64c4390,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                            ),
                                                                            start: 830,
                                                                            end: 844,
                                                                            as_str(): "Result::Err(x)",
                                                                        },
                                                                    },
                                                                    mutability: Immutable,
                                                                    return_type: TypeId(
                                                                        31794,
                                                                    ),
                                                                    type_ascription: TypeId(
                                                                        31794,
                                                                    ),
                                                                    type_ascription_span: None,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 842,
                                                            end: 843,
                                                            as_str(): "x",
                                                        },
                                                    },
                                                    TyAstNode {
                                                        content: ImplicitReturnExpression(
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
                                                                    src (ptr): 0x00007fb0f64c4390,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                    ),
                                                                    start: 859,
                                                                    end: 860,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb0f64c4390,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                            ),
                                                            start: 859,
                                                            end: 860,
                                                            as_str(): "3",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 849,
                                            end: 866,
                                            as_str(): "{\n        3\n    }",
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
                                                                            43,
                                                                        ),
                                                                    ),
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0f64c4390,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                        ),
                                                                        start: 882,
                                                                        end: 884,
                                                                        as_str(): "43",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb0f64c4390,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                                ),
                                                                start: 882,
                                                                end: 884,
                                                                as_str(): "43",
                                                            },
                                                        },
                                                    ],
                                                },
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 872,
                                                end: 890,
                                                as_str(): "{\n        43\n    }",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb0f64c4390,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                    ),
                                    start: 849,
                                    end: 866,
                                    as_str(): "{\n        3\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31791,
                            ),
                            type_ascription: TypeId(
                                31791,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 808,
                    end: 891,
                    as_str(): "let result_2 = if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    };",
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
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 905,
                                            end: 906,
                                            as_str(): "+",
                                        },
                                        is_raw_ident: false,
                                    },
                                    BaseIdent {
                                        name_override_opt: Some(
                                            "ops",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 905,
                                            end: 906,
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
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 905,
                                        end: 906,
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
                                            src (ptr): 0x00007fb1094e94d0,
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
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 724,
                                                    end: 732,
                                                    as_str(): "result_1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 896,
                                                end: 904,
                                                as_str(): "result_1",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31779,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 896,
                                            end: 904,
                                            as_str(): "result_1",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1094e94d0,
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
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb0f64c4390,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                    ),
                                                    start: 812,
                                                    end: 820,
                                                    as_str(): "result_2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb0f64c4390,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                                ),
                                                start: 907,
                                                end: 915,
                                                as_str(): "result_2",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31791,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb0f64c4390,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                            ),
                                            start: 907,
                                            end: 915,
                                            as_str(): "result_2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13429,
                                Span {
                                    src (ptr): 0x00007fb1094e94d0,
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
                                        src (ptr): 0x00007fb0f64c4390,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                                        ),
                                        start: 905,
                                        end: 906,
                                        as_str(): "+",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb0f64c4390,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                            ),
                            start: 896,
                            end: 915,
                            as_str(): "result_1 + result_2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb0f64c4390,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
                    ),
                    start: 896,
                    end: 915,
                    as_str(): "result_1 + result_2",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb0f64c4390,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
        ),
        start: 224,
        end: 917,
        as_str(): "fn main() -> u64 {\n    let a = Result::Ok::<u64, u64>(100);\n    let b = if let Result::Ok(y) = a { y + 10 } else { 1 };\n    assert(b == 110);\n\n    let sender = Identity::Address(B1);\n    if let Identity::Address(addr1) = sender {\n        match sender {\n            Identity::Address(addr2) => {\n                assert(addr1 == addr2);\n            }\n            _ => {\n                assert(false);\n            }\n        }\n    };\n\n    let x: Result<u64, u64> = Result::Ok::<u64, u64>(5u64);\n\n    let result_1 = if let Result::Ok(x) = x {\n        100\n    } else {\n        1\n    };\n    let result_2 = if let Result::Err(x) = x {\n        3\n    } else {\n        43\n    };\n    result_1 + result_2\n}",
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
        src (ptr): 0x00007fb0f64c4390,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRXXoLei/enum_if_let/src/main.sw",
        ),
        start: 237,
        end: 240,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

