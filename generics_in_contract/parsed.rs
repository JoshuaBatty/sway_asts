[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
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
                    attributes: {},
                    fields: [
                        StructField {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 37,
                                        end: 43,
                                        as_str(): "Option",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                31628,
                                            ),
                                            initial_type_id: TypeId(
                                                31628,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0e6142520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                ),
                                                start: 44,
                                                end: 52,
                                                as_str(): "Identity",
                                            },
                                        },
                                    ],
                                ),
                            },
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0e6142520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
            ),
            start: 11,
            end: 56,
            as_str(): "struct Game {\n    winner: Option<Identity>,\n}",
        },
    },
    AstNode {
        content: Declaration(
            AbiDeclaration(
                AbiDeclaration {
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
                        TraitFn {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0e6142520,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                    ),
                                    start: 103,
                                    end: 111,
                                    as_str(): "new_game",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {
                                Storage: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0e6142520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                ),
                                                start: 80,
                                                end: 87,
                                                as_str(): "storage",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6142520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                    ),
                                                    start: 88,
                                                    end: 93,
                                                    as_str(): "write",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0e6142520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                            ),
                                            start: 78,
                                            end: 95,
                                            as_str(): "#[storage(write)]",
                                        },
                                    },
                                ],
                            },
                            purity: Writes,
                            parameters: [],
                            return_type: Tuple(
                                [],
                            ),
                            return_type_span: Span {
                                src (ptr): 0x00007fe0e6142520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                ),
                                start: 100,
                                end: 113,
                                as_str(): "fn new_game()",
                            },
                        },
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0e6142520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
            ),
            start: 58,
            end: 116,
            as_str(): "abi TicTacToe {\n    #[storage(write)]\n    fn new_game();\n}",
        },
    },
    AstNode {
        content: Declaration(
            StorageDeclaration(
                StorageDeclaration {
                    attributes: {},
                    fields: [
                        StorageField {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 138,
                                        end: 142,
                                        as_str(): "Game",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            type_info_span: Span {
                                src (ptr): 0x00007fe0e6142520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                ),
                                start: 138,
                                end: 142,
                                as_str(): "Game",
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
                            initializer: Expression {
                                kind: Struct(
                                    StructExpression {
                                        call_path_binding: TypeBinding {
                                            inner: CallPath {
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0e6142520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                        ),
                                                        start: 145,
                                                        end: 149,
                                                        as_str(): "Game",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: false,
                                            },
                                            type_arguments: [],
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
                                        fields: [
                                            StructExpressionField {
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
                                                value: Expression {
                                                    kind: DelineatedPath(
                                                        DelineatedPathExpression {
                                                            call_path_binding: TypeBinding {
                                                                inner: CallPath {
                                                                    prefixes: [
                                                                        BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                ),
                                                                                start: 160,
                                                                                end: 166,
                                                                                as_str(): "Option",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    ],
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0e6142520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                            ),
                                                                            start: 168,
                                                                            end: 172,
                                                                            as_str(): "None",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    is_absolute: false,
                                                                },
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
                                                            args: None,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0e6142520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                        ),
                                                        start: 160,
                                                        end: 172,
                                                        as_str(): "Option::None",
                                                    },
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6142520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                    ),
                                                    start: 152,
                                                    end: 172,
                                                    as_str(): "winner: Option::None",
                                                },
                                            },
                                        ],
                                    },
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
                        },
                        StorageField {
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
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 193,
                                        end: 203,
                                        as_str(): "StorageMap",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                21,
                                            ),
                                            initial_type_id: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0e6142520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                ),
                                                start: 204,
                                                end: 207,
                                                as_str(): "u64",
                                            },
                                        },
                                        TypeArgument {
                                            type_id: TypeId(
                                                31630,
                                            ),
                                            initial_type_id: TypeId(
                                                31630,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0e6142520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                ),
                                                start: 209,
                                                end: 225,
                                                as_str(): "Option<Identity>",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_info_span: Span {
                                src (ptr): 0x00007fe0e6142520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                ),
                                start: 193,
                                end: 203,
                                as_str(): "StorageMap",
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
                            initializer: Expression {
                                kind: Struct(
                                    StructExpression {
                                        call_path_binding: TypeBinding {
                                            inner: CallPath {
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0e6142520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                        ),
                                                        start: 229,
                                                        end: 239,
                                                        as_str(): "StorageMap",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: false,
                                            },
                                            type_arguments: [],
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
                                        fields: [],
                                    },
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
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0e6142520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
            ),
            start: 118,
            end: 245,
            as_str(): "storage {\n    game: Game = Game { winner: Option::None },\n    game_boards: StorageMap<u64, Option<Identity>> = StorageMap {},\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0e6142520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                ),
                                start: 252,
                                end: 261,
                                as_str(): "TicTacToe",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Contract,
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0e6142520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                        ),
                        start: 266,
                        end: 274,
                        as_str(): "Contract",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Writes,
                            attributes: {
                                Storage: [
                                    Attribute {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0e6142520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                ),
                                                start: 283,
                                                end: 290,
                                                as_str(): "storage",
                                            },
                                            is_raw_ident: false,
                                        },
                                        args: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6142520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                    ),
                                                    start: 291,
                                                    end: 296,
                                                    as_str(): "write",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0e6142520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                            ),
                                            start: 281,
                                            end: 298,
                                            as_str(): "#[storage(write)]",
                                        },
                                    },
                                ],
                            },
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0e6142520,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                    ),
                                    start: 306,
                                    end: 314,
                                    as_str(): "new_game",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: Expression(
                                            Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0e6142520,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                        ),
                                                                        start: 347,
                                                                        end: 353,
                                                                        as_str(): "insert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 347,
                                                                end: 353,
                                                                as_str(): "insert",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: StorageAccess(
                                                                    StorageAccessExpression {
                                                                        field_names: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                    ),
                                                                                    start: 335,
                                                                                    end: 346,
                                                                                    as_str(): "game_boards",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 327,
                                                                    end: 346,
                                                                    as_str(): "storage.game_boards",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 354,
                                                                    end: 355,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: DelineatedPath(
                                                                    DelineatedPathExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 357,
                                                                                            end: 363,
                                                                                            as_str(): "Option",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0e6142520,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                        ),
                                                                                        start: 365,
                                                                                        end: 369,
                                                                                        as_str(): "None",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [
                                                                                TypeArgument {
                                                                                    type_id: TypeId(
                                                                                        31631,
                                                                                    ),
                                                                                    initial_type_id: TypeId(
                                                                                        31631,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0e6142520,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                        ),
                                                                                        start: 372,
                                                                                        end: 380,
                                                                                        as_str(): "Identity",
                                                                                    },
                                                                                },
                                                                            ],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                ),
                                                                                start: 365,
                                                                                end: 381,
                                                                                as_str(): "None::<Identity>",
                                                                            },
                                                                        },
                                                                        args: None,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 357,
                                                                    end: 381,
                                                                    as_str(): "Option::None::<Identity>",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6142520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                    ),
                                                    start: 327,
                                                    end: 382,
                                                    as_str(): "storage.game_boards.insert(1, Option::None::<Identity>)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0e6142520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                            ),
                                            start: 327,
                                            end: 382,
                                            as_str(): "storage.game_boards.insert(1, Option::None::<Identity>)",
                                        },
                                    },
                                    AstNode {
                                        content: Expression(
                                            Expression {
                                                kind: MethodApplication(
                                                    MethodApplicationExpression {
                                                        method_name_binding: TypeBinding {
                                                            inner: FromModule {
                                                                method_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0e6142520,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                        ),
                                                                        start: 412,
                                                                        end: 418,
                                                                        as_str(): "insert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 412,
                                                                end: 418,
                                                                as_str(): "insert",
                                                            },
                                                        },
                                                        contract_call_params: [],
                                                        arguments: [
                                                            Expression {
                                                                kind: StorageAccess(
                                                                    StorageAccessExpression {
                                                                        field_names: [
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                    ),
                                                                                    start: 400,
                                                                                    end: 411,
                                                                                    as_str(): "game_boards",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 392,
                                                                    end: 411,
                                                                    as_str(): "storage.game_boards",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 419,
                                                                    end: 420,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: DelineatedPath(
                                                                    DelineatedPathExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 422,
                                                                                            end: 428,
                                                                                            as_str(): "Option",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0e6142520,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                        ),
                                                                                        start: 430,
                                                                                        end: 434,
                                                                                        as_str(): "None",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                ),
                                                                                start: 430,
                                                                                end: 434,
                                                                                as_str(): "None",
                                                                            },
                                                                        },
                                                                        args: None,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 422,
                                                                    end: 434,
                                                                    as_str(): "Option::None",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6142520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                    ),
                                                    start: 392,
                                                    end: 435,
                                                    as_str(): "storage.game_boards.insert(1, Option::None)",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0e6142520,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                            ),
                                            start: 392,
                                            end: 435,
                                            as_str(): "storage.game_boards.insert(1, Option::None)",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0e6142520,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                    ),
                                    start: 317,
                                    end: 442,
                                    as_str(): "{\n        storage.game_boards.insert(1, Option::None::<Identity>);\n        storage.game_boards.insert(1, Option::None);\n    }",
                                },
                            },
                            parameters: [],
                            span: Span {
                                src (ptr): 0x00007fe0e6142520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                ),
                                start: 303,
                                end: 442,
                                as_str(): "fn new_game() {\n        storage.game_boards.insert(1, Option::None::<Identity>);\n        storage.game_boards.insert(1, Option::None);\n    }",
                            },
                            return_type: Tuple(
                                [],
                            ),
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0e6142520,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                ),
                                start: 303,
                                end: 316,
                                as_str(): "fn new_game()",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0e6142520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                        ),
                        start: 247,
                        end: 444,
                        as_str(): "impl TicTacToe for Contract {\n    #[storage(write)]\n    fn new_game() {\n        storage.game_boards.insert(1, Option::None::<Identity>);\n        storage.game_boards.insert(1, Option::None);\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0e6142520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
            ),
            start: 247,
            end: 444,
            as_str(): "impl TicTacToe for Contract {\n    #[storage(write)]\n    fn new_game() {\n        storage.game_boards.insert(1, Option::None::<Identity>);\n        storage.game_boards.insert(1, Option::None);\n    }\n}",
        },
    },
]
