TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 16,
            end: 29,
            as_str(): "GenericStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0ef5d6490,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                    ),
                    start: 39,
                    end: 40,
                    as_str(): "x",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31646,
            ),
            initial_type_id: TypeId(
                31647,
            ),
            span: Span {
                src (ptr): 0x00007fe0ef5d6490,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                ),
                start: 39,
                end: 42,
                as_str(): "x:T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0ef5d6490,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                ),
                start: 41,
                end: 42,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(31646),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0ef5d6490,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
        ),
        start: 9,
        end: 44,
        as_str(): "struct GenericStruct<T> {\n    x:T\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13670,
        Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 46,
            end: 347,
            as_str(): "impl<T, E> Option<Result<T, E>> {\n    pub fn transpose(self) -> Result<Option<T>, E> {\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        13865,
        Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 349,
            end: 614,
            as_str(): "impl<T> GenericStruct<Option<T>> {\n    pub fn transpose(self) -> Option<GenericStruct<T>> {\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 619,
            end: 623,
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
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 644,
                                    end: 645,
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
                                                src (ptr): 0x00007fe0fa399070,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                ),
                                                start: 2471,
                                                end: 2477,
                                                as_str(): "Option",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(32930),
                                        ],
                                        attributes: {
                                            DocComment: [
                                                Attribute {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "doc-comment",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fa399070,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 2386,
                                                            end: 2461,
                                                            as_str(): "/// The `Option` type. See [the module level documentation](self) for more.",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    args: [
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fa399070,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                ),
                                                                start: 2389,
                                                                end: 2461,
                                                                as_str(): " The `Option` type. See [the module level documentation](self) for more.",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fa399070,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                        ),
                                                        start: 2386,
                                                        end: 2461,
                                                        as_str(): "/// The `Option` type. See [the module level documentation](self) for more.",
                                                    },
                                                },
                                            ],
                                        },
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fa399070,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                        ),
                                                        start: 2505,
                                                        end: 2509,
                                                        as_str(): "None",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7589,
                                                ),
                                                initial_type_id: TypeId(
                                                    7588,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fa399070,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2511,
                                                    end: 2513,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fa399070,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2505,
                                                    end: 2513,
                                                    as_str(): "None: ()",
                                                },
                                                attributes: {
                                                    DocComment: [
                                                        Attribute {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "doc-comment",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fa399070,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 2487,
                                                                    end: 2500,
                                                                    as_str(): "/// No value.",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            args: [
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fa399070,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2490,
                                                                        end: 2500,
                                                                        as_str(): " No value.",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fa399070,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                ),
                                                                start: 2487,
                                                                end: 2500,
                                                                as_str(): "/// No value.",
                                                            },
                                                        },
                                                    ],
                                                },
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fa399070,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                        ),
                                                        start: 2551,
                                                        end: 2555,
                                                        as_str(): "Some",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    32930,
                                                ),
                                                initial_type_id: TypeId(
                                                    7590,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fa399070,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2557,
                                                    end: 2558,
                                                    as_str(): "T",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fa399070,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2551,
                                                    end: 2558,
                                                    as_str(): "Some: T",
                                                },
                                                attributes: {
                                                    DocComment: [
                                                        Attribute {
                                                            name: BaseIdent {
                                                                name_override_opt: Some(
                                                                    "doc-comment",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fa399070,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 2519,
                                                                    end: 2546,
                                                                    as_str(): "/// Some value of type `T`.",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            args: [
                                                                BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fa399070,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2522,
                                                                        end: 2546,
                                                                        as_str(): " Some value of type `T`.",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            ],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fa399070,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                ),
                                                                start: 2519,
                                                                end: 2546,
                                                                as_str(): "/// Some value of type `T`.",
                                                            },
                                                        },
                                                    ],
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fa399070,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                            ),
                                            start: 2462,
                                            end: 2561,
                                            as_str(): "pub enum Option<T> {\n    /// No value.\n    None: (),\n    /// Some value of type `T`.\n    Some: T,\n}",
                                        },
                                        visibility: Public,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fa399070,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                            ),
                                            start: 2551,
                                            end: 2555,
                                            as_str(): "Some",
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
                                                            src (ptr): 0x00007fe0f65610b0,
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
                                                        T: TypeId(32974),
                                                        E: TypeId(32975),
                                                    ],
                                                    attributes: {
                                                        DocComment: [
                                                            Attribute {
                                                                name: BaseIdent {
                                                                    name_override_opt: Some(
                                                                        "doc-comment",
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f65610b0,
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
                                                                            src (ptr): 0x00007fe0f65610b0,
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
                                                                    src (ptr): 0x00007fe0f65610b0,
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
                                                                        src (ptr): 0x00007fe0f65610b0,
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
                                                                            src (ptr): 0x00007fe0f65610b0,
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
                                                                    src (ptr): 0x00007fe0f65610b0,
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
                                                                    src (ptr): 0x00007fe0f65610b0,
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
                                                                32974,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                7487,
                                                            ),
                                                            type_span: Span {
                                                                src (ptr): 0x00007fe0f65610b0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1866,
                                                                end: 1867,
                                                                as_str(): "T",
                                                            },
                                                            tag: 0,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f65610b0,
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
                                                                                src (ptr): 0x00007fe0f65610b0,
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
                                                                                    src (ptr): 0x00007fe0f65610b0,
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
                                                                            src (ptr): 0x00007fe0f65610b0,
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
                                                                    src (ptr): 0x00007fe0f65610b0,
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
                                                                32975,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                7488,
                                                            ),
                                                            type_span: Span {
                                                                src (ptr): 0x00007fe0f65610b0,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                ),
                                                                start: 1911,
                                                                end: 1912,
                                                                as_str(): "E",
                                                            },
                                                            tag: 1,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f65610b0,
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
                                                                                src (ptr): 0x00007fe0f65610b0,
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
                                                                                    src (ptr): 0x00007fe0f65610b0,
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
                                                                            src (ptr): 0x00007fe0f65610b0,
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
                                                        src (ptr): 0x00007fe0f65610b0,
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
                                                        src (ptr): 0x00007fe0f65610b0,
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 697,
                                                            end: 698,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                                enum_instantiation_span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 686,
                                                    end: 692,
                                                    as_str(): "Result",
                                                },
                                                variant_instantiation_span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 694,
                                                    end: 696,
                                                    as_str(): "Ok",
                                                },
                                                type_binding: TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 686,
                                                        end: 696,
                                                        as_str(): "Result::Ok",
                                                    },
                                                },
                                            },
                                            return_type: TypeId(
                                                33003,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0ef5d6490,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                ),
                                                start: 694,
                                                end: 696,
                                                as_str(): "Ok",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 673,
                                        end: 679,
                                        as_str(): "Option",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 681,
                                        end: 685,
                                        as_str(): "Some",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 673,
                                            end: 685,
                                            as_str(): "Option::Some",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    33004,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 681,
                                    end: 685,
                                    as_str(): "Some",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33004,
                            ),
                            type_ascription: TypeId(
                                32771,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 647,
                                    end: 670,
                                    as_str(): "Option<Result<u64, u8>>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0ef5d6490,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                    ),
                    start: 640,
                    end: 701,
                    as_str(): "let y: Option<Result<u64, u8>> = Option::Some(Result::Ok(5));",
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
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 706,
                                        end: 712,
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
                                            src (ptr): 0x00007fe0f6909aa0,
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 745,
                                                            end: 747,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 745,
                                                            end: 747,
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
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 745,
                                                        end: 747,
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
                                                            src (ptr): 0x00007fe0f68a84a0,
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
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 736,
                                                                        end: 742,
                                                                        as_str(): "unwrap",
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
                                                                            src (ptr): 0x00007fe0fa399070,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 4635,
                                                                            end: 4639,
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
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 727,
                                                                                        end: 733,
                                                                                        as_str(): "unwrap",
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
                                                                                            src (ptr): 0x00007fe0f65610b0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                            ),
                                                                                            start: 4032,
                                                                                            end: 4036,
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
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 715,
                                                                                                        end: 724,
                                                                                                        as_str(): "transpose",
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
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 101,
                                                                                                            end: 105,
                                                                                                            as_str(): "self",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 644,
                                                                                                                    end: 645,
                                                                                                                    as_str(): "y",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 713,
                                                                                                                end: 714,
                                                                                                                as_str(): "y",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            33004,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 713,
                                                                                                            end: 714,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            function_decl_id: DeclId(
                                                                                                13986,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 84,
                                                                                                    end: 345,
                                                                                                    as_str(): "pub fn transpose(self) -> Result<Option<T>, E> {\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }",
                                                                                                },
                                                                                            ),
                                                                                            self_state_idx: None,
                                                                                            selector: None,
                                                                                            type_binding: Some(
                                                                                                TypeBinding {
                                                                                                    inner: (),
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 715,
                                                                                                        end: 724,
                                                                                                        as_str(): "transpose",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            32819,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 713,
                                                                                            end: 726,
                                                                                            as_str(): "y.transpose()",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                14045,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe0f65610b0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 4018,
                                                                                    end: 4161,
                                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Result::Ok(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 727,
                                                                                        end: 733,
                                                                                        as_str(): "unwrap",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            32818,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 713,
                                                                            end: 735,
                                                                            as_str(): "y.transpose().unwrap()",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                14109,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fa399070,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 4621,
                                                                    end: 4766,
                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 736,
                                                                        end: 742,
                                                                        as_str(): "unwrap",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 713,
                                                            end: 744,
                                                            as_str(): "y.transpose().unwrap().unwrap()",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f68a84a0,
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
                                                                5,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 748,
                                                            end: 749,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                14110,
                                                Span {
                                                    src (ptr): 0x00007fe0f68a84a0,
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
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 745,
                                                        end: 747,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 713,
                                            end: 749,
                                            as_str(): "y.transpose().unwrap().unwrap() == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                14111,
                                Span {
                                    src (ptr): 0x00007fe0f6909aa0,
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
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 706,
                                        end: 712,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            33162,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0ef5d6490,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                            ),
                            start: 706,
                            end: 750,
                            as_str(): "assert(y.transpose().unwrap().unwrap() == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0ef5d6490,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                    ),
                    start: 706,
                    end: 750,
                    as_str(): "assert(y.transpose().unwrap().unwrap() == 5)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 761,
                                    end: 762,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 29,
                                            as_str(): "GenericStruct",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 808,
                                                    end: 809,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: EnumInstantiation {
                                                    enum_decl: TyEnumDeclaration {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fa399070,
                                                                path: Some(
                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                ),
                                                                start: 2471,
                                                                end: 2477,
                                                                as_str(): "Option",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        type_parameters: [
                                                            T: TypeId(33331),
                                                        ],
                                                        attributes: {
                                                            DocComment: [
                                                                Attribute {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "doc-comment",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fa399070,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 2386,
                                                                            end: 2461,
                                                                            as_str(): "/// The `Option` type. See [the module level documentation](self) for more.",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    args: [
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fa399070,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                ),
                                                                                start: 2389,
                                                                                end: 2461,
                                                                                as_str(): " The `Option` type. See [the module level documentation](self) for more.",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fa399070,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2386,
                                                                        end: 2461,
                                                                        as_str(): "/// The `Option` type. See [the module level documentation](self) for more.",
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        variants: [
                                                            TyEnumVariant {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fa399070,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2505,
                                                                        end: 2509,
                                                                        as_str(): "None",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    7589,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    7588,
                                                                ),
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0fa399070,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 2511,
                                                                    end: 2513,
                                                                    as_str(): "()",
                                                                },
                                                                tag: 0,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fa399070,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 2505,
                                                                    end: 2513,
                                                                    as_str(): "None: ()",
                                                                },
                                                                attributes: {
                                                                    DocComment: [
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fa399070,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                    ),
                                                                                    start: 2487,
                                                                                    end: 2500,
                                                                                    as_str(): "/// No value.",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: [
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fa399070,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                        ),
                                                                                        start: 2490,
                                                                                        end: 2500,
                                                                                        as_str(): " No value.",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fa399070,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                ),
                                                                                start: 2487,
                                                                                end: 2500,
                                                                                as_str(): "/// No value.",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                            TyEnumVariant {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fa399070,
                                                                        path: Some(
                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                        ),
                                                                        start: 2551,
                                                                        end: 2555,
                                                                        as_str(): "Some",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    33331,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    7590,
                                                                ),
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0fa399070,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 2557,
                                                                    end: 2558,
                                                                    as_str(): "T",
                                                                },
                                                                tag: 1,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fa399070,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                    ),
                                                                    start: 2551,
                                                                    end: 2558,
                                                                    as_str(): "Some: T",
                                                                },
                                                                attributes: {
                                                                    DocComment: [
                                                                        Attribute {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "doc-comment",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fa399070,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                    ),
                                                                                    start: 2519,
                                                                                    end: 2546,
                                                                                    as_str(): "/// Some value of type `T`.",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            args: [
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fa399070,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                        ),
                                                                                        start: 2522,
                                                                                        end: 2546,
                                                                                        as_str(): " Some value of type `T`.",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fa399070,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                ),
                                                                                start: 2519,
                                                                                end: 2546,
                                                                                as_str(): "/// Some value of type `T`.",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fa399070,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 2462,
                                                            end: 2561,
                                                            as_str(): "pub enum Option<T> {\n    /// No value.\n    None: (),\n    /// Some value of type `T`.\n    Some: T,\n}",
                                                        },
                                                        visibility: Public,
                                                    },
                                                    variant_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fa399070,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                            ),
                                                            start: 2551,
                                                            end: 2555,
                                                            as_str(): "Some",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    tag: 1,
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
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 824,
                                                                end: 825,
                                                                as_str(): "5",
                                                            },
                                                        },
                                                    ),
                                                    enum_instantiation_span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 811,
                                                        end: 817,
                                                        as_str(): "Option",
                                                    },
                                                    variant_instantiation_span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 819,
                                                        end: 823,
                                                        as_str(): "Some",
                                                    },
                                                    type_binding: TypeBinding {
                                                        inner: (),
                                                        type_arguments: [],
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 811,
                                                            end: 823,
                                                            as_str(): "Option::Some",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    33376,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 819,
                                                    end: 823,
                                                    as_str(): "Some",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 793,
                                        end: 806,
                                        as_str(): "GenericStruct",
                                    },
                                },
                                return_type: TypeId(
                                    33329,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 793,
                                    end: 827,
                                    as_str(): "GenericStruct{ x: Option::Some(5)}",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33329,
                            ),
                            type_ascription: TypeId(
                                33208,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 764,
                                    end: 790,
                                    as_str(): "GenericStruct<Option<u64>>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0ef5d6490,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                    ),
                    start: 757,
                    end: 828,
                    as_str(): "let y: GenericStruct<Option<u64>> = GenericStruct{ x: Option::Some(5)};",
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
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 833,
                                        end: 839,
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
                                            src (ptr): 0x00007fe0f6909aa0,
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
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 865,
                                                            end: 867,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 865,
                                                            end: 867,
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
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 865,
                                                        end: 867,
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
                                                            src (ptr): 0x00007fe0f68a84a0,
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
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [],
                                                                        suffix: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 854,
                                                                                end: 860,
                                                                                as_str(): "unwrap",
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
                                                                                    src (ptr): 0x00007fe0fa399070,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                                    ),
                                                                                    start: 4635,
                                                                                    end: 4639,
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
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 842,
                                                                                                end: 851,
                                                                                                as_str(): "transpose",
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
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 405,
                                                                                                    end: 409,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 761,
                                                                                                            end: 762,
                                                                                                            as_str(): "y",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 840,
                                                                                                        end: 841,
                                                                                                        as_str(): "y",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    33329,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 840,
                                                                                                    end: 841,
                                                                                                    as_str(): "y",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        14287,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 388,
                                                                                            end: 612,
                                                                                            as_str(): "pub fn transpose(self) -> Option<GenericStruct<T>> {\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }",
                                                                                        },
                                                                                    ),
                                                                                    self_state_idx: None,
                                                                                    selector: None,
                                                                                    type_binding: Some(
                                                                                        TypeBinding {
                                                                                            inner: (),
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 842,
                                                                                                end: 851,
                                                                                                as_str(): "transpose",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    33215,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 840,
                                                                                    end: 853,
                                                                                    as_str(): "y.transpose()",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        14309,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0fa399070,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                                            ),
                                                                            start: 4621,
                                                                            end: 4766,
                                                                            as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Option::Some(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                        },
                                                                    ),
                                                                    self_state_idx: None,
                                                                    selector: None,
                                                                    type_binding: Some(
                                                                        TypeBinding {
                                                                            inner: (),
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 854,
                                                                                end: 860,
                                                                                as_str(): "unwrap",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    33214,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 840,
                                                                    end: 862,
                                                                    as_str(): "y.transpose().unwrap()",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 39,
                                                                        end: 40,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    21,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    31647,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 39,
                                                                    end: 42,
                                                                    as_str(): "x:T",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 41,
                                                                    end: 42,
                                                                    as_str(): "T",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 863,
                                                                end: 864,
                                                                as_str(): "x",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                33214,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 840,
                                                            end: 864,
                                                            as_str(): "y.transpose().unwrap().x",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f68a84a0,
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
                                                                5,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 868,
                                                            end: 869,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                14310,
                                                Span {
                                                    src (ptr): 0x00007fe0f68a84a0,
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
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 865,
                                                        end: 867,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 840,
                                            end: 869,
                                            as_str(): "y.transpose().unwrap().x == 5",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                14311,
                                Span {
                                    src (ptr): 0x00007fe0f6909aa0,
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
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 833,
                                        end: 839,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            33426,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0ef5d6490,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                            ),
                            start: 833,
                            end: 870,
                            as_str(): "assert(y.transpose().unwrap().x == 5)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0ef5d6490,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                    ),
                    start: 833,
                    end: 870,
                    as_str(): "assert(y.transpose().unwrap().x == 5)",
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
                            src (ptr): 0x00007fe0ef5d6490,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                            ),
                            start: 877,
                            end: 881,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0ef5d6490,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                    ),
                    start: 877,
                    end: 881,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0ef5d6490,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
        ),
        start: 616,
        end: 883,
        as_str(): "fn main() -> bool {\n    let y: Option<Result<u64, u8>> = Option::Some(Result::Ok(5));\n    assert(y.transpose().unwrap().unwrap() == 5);\n\n    let y: GenericStruct<Option<u64>> = GenericStruct{ x: Option::Some(5)};\n    assert(y.transpose().unwrap().x == 5);\n\n    true\n}",
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
        src (ptr): 0x00007fe0ef5d6490,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
        ),
        start: 629,
        end: 633,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

