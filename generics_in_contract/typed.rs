TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0e6142520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
            ),
            start: 18,
            end: 22,
            as_str(): "Game",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0e6142520,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                    ),
                    start: 29,
                    end: 35,
                    as_str(): "winner",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31635,
            ),
            initial_type_id: TypeId(
                31634,
            ),
            span: Span {
                src (ptr): 0x00007fe0e6142520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                ),
                start: 29,
                end: 53,
                as_str(): "winner: Option<Identity>",
            },
            type_span: Span {
                src (ptr): 0x00007fe0e6142520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                ),
                start: 37,
                end: 53,
                as_str(): "Option<Identity>",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0e6142520,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
        ),
        start: 11,
        end: 56,
        as_str(): "struct Game {\n    winner: Option<Identity>,\n}",
    },
    attributes: {},
}
TyAbiDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0e6142520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
            ),
            start: 62,
            end: 71,
            as_str(): "TicTacToe",
        },
        is_raw_ident: false,
    },
    interface_surface: [
        DeclId(
            13336,
            Span {
                src (ptr): 0x00007fe0e6142520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                ),
                start: 103,
                end: 111,
                as_str(): "new_game",
            },
        ),
    ],
    methods: [],
    span: Span {
        src (ptr): 0x00007fe0e6142520,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
        ),
        start: 58,
        end: 116,
        as_str(): "abi TicTacToe {\n    #[storage(write)]\n    fn new_game();\n}",
    },
    attributes: {},
}
TyStorageDeclaration {
    fields: [
        TyStorageField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0e6142520,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                    ),
                    start: 132,
                    end: 136,
                    as_str(): "game",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31680,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0e6142520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                ),
                start: 138,
                end: 142,
                as_str(): "Game",
            },
            initializer: TyExpression {
                expression: StructExpression {
                    struct_name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0e6142520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                            ),
                            start: 18,
                            end: 22,
                            as_str(): "Game",
                        },
                        is_raw_ident: false,
                    },
                    fields: [
                        TyStructExpressionField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0e6142520,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                    ),
                                    start: 152,
                                    end: 158,
                                    as_str(): "winner",
                                },
                                is_raw_ident: false,
                            },
                            value: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fb3873a0,
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
                                            T: TypeId(31766),
                                        ],
                                        attributes: {
                                            DocComment: [
                                                Attribute {
                                                    name: BaseIdent {
                                                        name_override_opt: Some(
                                                            "doc-comment",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb3873a0,
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
                                                                src (ptr): 0x00007fe0fb3873a0,
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
                                                        src (ptr): 0x00007fe0fb3873a0,
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
                                                        src (ptr): 0x00007fe0fb3873a0,
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
                                                    src (ptr): 0x00007fe0fb3873a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2511,
                                                    end: 2513,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb3873a0,
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
                                                                    src (ptr): 0x00007fe0fb3873a0,
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
                                                                        src (ptr): 0x00007fe0fb3873a0,
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
                                                                src (ptr): 0x00007fe0fb3873a0,
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
                                                        src (ptr): 0x00007fe0fb3873a0,
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
                                                    31766,
                                                ),
                                                initial_type_id: TypeId(
                                                    7590,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb3873a0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                                    ),
                                                    start: 2557,
                                                    end: 2558,
                                                    as_str(): "T",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb3873a0,
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
                                                                    src (ptr): 0x00007fe0fb3873a0,
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
                                                                        src (ptr): 0x00007fe0fb3873a0,
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
                                                                src (ptr): 0x00007fe0fb3873a0,
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
                                            src (ptr): 0x00007fe0fb3873a0,
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
                                            src (ptr): 0x00007fe0fb3873a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/option.sw",
                                            ),
                                            start: 2505,
                                            end: 2509,
                                            as_str(): "None",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 160,
                                        end: 166,
                                        as_str(): "Option",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 168,
                                        end: 172,
                                        as_str(): "None",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0e6142520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                            ),
                                            start: 168,
                                            end: 172,
                                            as_str(): "None",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31809,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0e6142520,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                    ),
                                    start: 168,
                                    end: 172,
                                    as_str(): "None",
                                },
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fe0e6142520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                        ),
                        start: 145,
                        end: 149,
                        as_str(): "Game",
                    },
                },
                return_type: TypeId(
                    31680,
                ),
                span: Span {
                    src (ptr): 0x00007fe0e6142520,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                    ),
                    start: 145,
                    end: 174,
                    as_str(): "Game { winner: Option::None }",
                },
            },
            span: Span {
                src (ptr): 0x00007fe0e6142520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                ),
                start: 132,
                end: 174,
                as_str(): "game: Game = Game { winner: Option::None }",
            },
            attributes: {},
        },
        TyStorageField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0e6142520,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                    ),
                    start: 180,
                    end: 191,
                    as_str(): "game_boards",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31855,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0e6142520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                ),
                start: 193,
                end: 203,
                as_str(): "StorageMap",
            },
            initializer: TyExpression {
                expression: StructExpression {
                    struct_name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0efd94700,
                            path: Some(
                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/storage.sw",
                            ),
                            start: 3248,
                            end: 3258,
                            as_str(): "StorageMap",
                        },
                        is_raw_ident: false,
                    },
                    fields: [],
                    span: Span {
                        src (ptr): 0x00007fe0e6142520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                        ),
                        start: 229,
                        end: 239,
                        as_str(): "StorageMap",
                    },
                },
                return_type: TypeId(
                    31920,
                ),
                span: Span {
                    src (ptr): 0x00007fe0e6142520,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                    ),
                    start: 229,
                    end: 242,
                    as_str(): "StorageMap {}",
                },
            },
            span: Span {
                src (ptr): 0x00007fe0e6142520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                ),
                start: 180,
                end: 242,
                as_str(): "game_boards: StorageMap<u64, Option<Identity>> = StorageMap {}",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0e6142520,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
        ),
        start: 118,
        end: 245,
        as_str(): "storage {\n    game: Game = Game { winner: Option::None },\n    game_boards: StorageMap<u64, Option<Identity>> = StorageMap {},\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13753,
        Span {
            src (ptr): 0x00007fe0e6142520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
            ),
            start: 247,
            end: 444,
            as_str(): "impl TicTacToe for Contract {\n    #[storage(write)]\n    fn new_game() {\n        storage.game_boards.insert(1, Option::None::<Identity>);\n        storage.game_boards.insert(1, Option::None);\n    }\n}",
        },
    ),
)

