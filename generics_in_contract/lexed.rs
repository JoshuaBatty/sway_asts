Some(
    LexedProgram {
        kind: Contract,
        root: LexedModule {
            tree: Module {
                kind: Contract {
                    contract_token: ContractToken {
                        span: Span {
                            src (ptr): 0x00007fe0e6142520,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                            ),
                            start: 0,
                            end: 8,
                            as_str(): "contract",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0e6142520,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                        ),
                        start: 8,
                        end: 9,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 11,
                                        end: 17,
                                        as_str(): "struct",
                                    },
                                },
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
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
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
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 36,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
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
                                                                    generics_opt: Some(
                                                                        (
                                                                            None,
                                                                            GenericArgs {
                                                                                parameters: AngleBrackets {
                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 43,
                                                                                            end: 44,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                    },
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                ),
                                                                                                                start: 44,
                                                                                                                end: 52,
                                                                                                                as_str(): "Identity",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                    suffix: [],
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 52,
                                                                                            end: 53,
                                                                                            as_str(): ">",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0e6142520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                        ),
                                                        start: 53,
                                                        end: 54,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 23,
                                        end: 56,
                                        as_str(): "{\n    winner: Option<Identity>,\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Abi(
                            ItemAbi {
                                abi_token: AbiToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 58,
                                        end: 61,
                                        as_str(): "abi",
                                    },
                                },
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
                                abi_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [
                                                    AttributeDecl {
                                                        hash_token: HashToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 78,
                                                                end: 79,
                                                                as_str(): "#",
                                                            },
                                                        },
                                                        attribute: SquareBrackets {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
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
                                                                        args: Some(
                                                                            Parens {
                                                                                inner: Punctuated {
                                                                                    value_separator_pairs: [],
                                                                                    final_value_opt: Some(
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
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                    ),
                                                                                    start: 87,
                                                                                    end: 94,
                                                                                    as_str(): "(write)",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 79,
                                                                end: 95,
                                                                as_str(): "[storage(write)]",
                                                            },
                                                        },
                                                    },
                                                ],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0e6142520,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                            ),
                                                            start: 100,
                                                            end: 102,
                                                            as_str(): "fn",
                                                        },
                                                    },
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
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: Static(
                                                            Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0e6142520,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                            ),
                                                            start: 111,
                                                            end: 113,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6142520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                    ),
                                                    start: 113,
                                                    end: 114,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 72,
                                        end: 116,
                                        as_str(): "{\n    #[storage(write)]\n    fn new_game();\n}",
                                    },
                                },
                                abi_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Storage(
                            ItemStorage {
                                storage_token: StorageToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 118,
                                        end: 125,
                                        as_str(): "storage",
                                    },
                                },
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: StorageField {
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
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 136,
                                                                end: 137,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
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
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 143,
                                                                end: 144,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Struct {
                                                            path: PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
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
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                                incomplete_suffix: false,
                                                            },
                                                            fields: Braces {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: Some(
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
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
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 158,
                                                                                            end: 159,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
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
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [
                                                                                                (
                                                                                                    DoubleColonToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                            ),
                                                                                                            start: 166,
                                                                                                            end: 168,
                                                                                                            as_str(): "::",
                                                                                                        },
                                                                                                    },
                                                                                                    PathExprSegment {
                                                                                                        name: BaseIdent {
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
                                                                                                        generics_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            incomplete_suffix: false,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 150,
                                                                    end: 174,
                                                                    as_str(): "{ winner: Option::None }",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0e6142520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                        ),
                                                        start: 174,
                                                        end: 175,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: StorageField {
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
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 191,
                                                                end: 192,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Path(
                                                            PathType {
                                                                root_opt: None,
                                                                prefix: PathTypeSegment {
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
                                                                    generics_opt: Some(
                                                                        (
                                                                            None,
                                                                            GenericArgs {
                                                                                parameters: AngleBrackets {
                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 203,
                                                                                            end: 204,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                    },
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [
                                                                                            (
                                                                                                Path(
                                                                                                    PathType {
                                                                                                        root_opt: None,
                                                                                                        prefix: PathTypeSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 204,
                                                                                                                    end: 207,
                                                                                                                    as_str(): "u64",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                    },
                                                                                                ),
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0e6142520,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                        ),
                                                                                                        start: 207,
                                                                                                        end: 208,
                                                                                                        as_str(): ",",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        final_value_opt: Some(
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                ),
                                                                                                                start: 209,
                                                                                                                end: 215,
                                                                                                                as_str(): "Option",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        generics_opt: Some(
                                                                                                            (
                                                                                                                None,
                                                                                                                GenericArgs {
                                                                                                                    parameters: AngleBrackets {
                                                                                                                        open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 215,
                                                                                                                                end: 216,
                                                                                                                                as_str(): "<",
                                                                                                                            },
                                                                                                                        },
                                                                                                                        inner: Punctuated {
                                                                                                                            value_separator_pairs: [],
                                                                                                                            final_value_opt: Some(
                                                                                                                                Path(
                                                                                                                                    PathType {
                                                                                                                                        root_opt: None,
                                                                                                                                        prefix: PathTypeSegment {
                                                                                                                                            name: BaseIdent {
                                                                                                                                                name_override_opt: None,
                                                                                                                                                span: Span {
                                                                                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                                                                                    path: Some(
                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                                                    ),
                                                                                                                                                    start: 216,
                                                                                                                                                    end: 224,
                                                                                                                                                    as_str(): "Identity",
                                                                                                                                                },
                                                                                                                                                is_raw_ident: false,
                                                                                                                                            },
                                                                                                                                            generics_opt: None,
                                                                                                                                        },
                                                                                                                                        suffix: [],
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                            ),
                                                                                                                        },
                                                                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 224,
                                                                                                                                end: 225,
                                                                                                                                as_str(): ">",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    suffix: [],
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 225,
                                                                                            end: 226,
                                                                                            as_str(): ">",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                        eq_token: EqToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0e6142520,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                ),
                                                                start: 227,
                                                                end: 228,
                                                                as_str(): "=",
                                                            },
                                                        },
                                                        initializer: Struct {
                                                            path: PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
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
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                                incomplete_suffix: false,
                                                            },
                                                            fields: Braces {
                                                                inner: Punctuated {
                                                                    value_separator_pairs: [],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0e6142520,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                    ),
                                                                    start: 240,
                                                                    end: 242,
                                                                    as_str(): "{}",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0e6142520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                        ),
                                                        start: 242,
                                                        end: 243,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 126,
                                        end: 245,
                                        as_str(): "{\n    game: Game = Game { winner: Option::None },\n    game_boards: StorageMap<u64, Option<Identity>> = StorageMap {},\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 247,
                                        end: 251,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: Some(
                                    (
                                        PathType {
                                            root_opt: None,
                                            prefix: PathTypeSegment {
                                                name: BaseIdent {
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
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0e6142520,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                ),
                                                start: 262,
                                                end: 265,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0e6142520,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                    ),
                                                    start: 266,
                                                    end: 274,
                                                    as_str(): "Contract",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [
                                                AttributeDecl {
                                                    hash_token: HashToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0e6142520,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                            ),
                                                            start: 281,
                                                            end: 282,
                                                            as_str(): "#",
                                                        },
                                                    },
                                                    attribute: SquareBrackets {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: Some(
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
                                                                    args: Some(
                                                                        Parens {
                                                                            inner: Punctuated {
                                                                                value_separator_pairs: [],
                                                                                final_value_opt: Some(
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
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                ),
                                                                                start: 290,
                                                                                end: 297,
                                                                                as_str(): "(write)",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0e6142520,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 298,
                                                            as_str(): "[storage(write)]",
                                                        },
                                                    },
                                                },
                                            ],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0e6142520,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                            ),
                                                            start: 303,
                                                            end: 305,
                                                            as_str(): "fn",
                                                        },
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
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: Static(
                                                            Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: None,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0e6142520,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                            ),
                                                            start: 314,
                                                            end: 316,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [
                                                            Expr {
                                                                expr: MethodCall {
                                                                    target: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 327,
                                                                                            end: 334,
                                                                                            as_str(): "storage",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                ),
                                                                                start: 334,
                                                                                end: 335,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
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
                                                                    },
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0e6142520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                            ),
                                                                            start: 346,
                                                                            end: 347,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
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
                                                                        generics_opt: None,
                                                                    },
                                                                    contract_args_opt: None,
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                    ),
                                                                                                    start: 354,
                                                                                                    end: 355,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 355,
                                                                                            end: 356,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
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
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0e6142520,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                        ),
                                                                                                        start: 363,
                                                                                                        end: 365,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
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
                                                                                                    generics_opt: Some(
                                                                                                        (
                                                                                                            DoubleColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 369,
                                                                                                                    end: 371,
                                                                                                                    as_str(): "::",
                                                                                                                },
                                                                                                            },
                                                                                                            GenericArgs {
                                                                                                                parameters: AngleBrackets {
                                                                                                                    open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 371,
                                                                                                                            end: 372,
                                                                                                                            as_str(): "<",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    inner: Punctuated {
                                                                                                                        value_separator_pairs: [],
                                                                                                                        final_value_opt: Some(
                                                                                                                            Path(
                                                                                                                                PathType {
                                                                                                                                    root_opt: None,
                                                                                                                                    prefix: PathTypeSegment {
                                                                                                                                        name: BaseIdent {
                                                                                                                                            name_override_opt: None,
                                                                                                                                            span: Span {
                                                                                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 372,
                                                                                                                                                end: 380,
                                                                                                                                                as_str(): "Identity",
                                                                                                                                            },
                                                                                                                                            is_raw_ident: false,
                                                                                                                                        },
                                                                                                                                        generics_opt: None,
                                                                                                                                    },
                                                                                                                                    suffix: [],
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    },
                                                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 380,
                                                                                                                            end: 381,
                                                                                                                            as_str(): ">",
                                                                                                                        },
                                                                                                                    },
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ),
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        incomplete_suffix: false,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0e6142520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                            ),
                                                                            start: 353,
                                                                            end: 382,
                                                                            as_str(): "(1, Option::None::<Identity>)",
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0e6142520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                            ),
                                                                            start: 382,
                                                                            end: 383,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            Expr {
                                                                expr: MethodCall {
                                                                    target: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 392,
                                                                                            end: 399,
                                                                                            as_str(): "storage",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0e6142520,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                ),
                                                                                start: 399,
                                                                                end: 400,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
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
                                                                    },
                                                                    dot_token: DotToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0e6142520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                            ),
                                                                            start: 411,
                                                                            end: 412,
                                                                            as_str(): ".",
                                                                        },
                                                                    },
                                                                    path_seg: PathExprSegment {
                                                                        name: BaseIdent {
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
                                                                        generics_opt: None,
                                                                    },
                                                                    contract_args_opt: None,
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0e6142520,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                    ),
                                                                                                    start: 419,
                                                                                                    end: 420,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0e6142520,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                            ),
                                                                                            start: 420,
                                                                                            end: 421,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
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
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0e6142520,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                                                        ),
                                                                                                        start: 428,
                                                                                                        end: 430,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
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
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        incomplete_suffix: false,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0e6142520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                            ),
                                                                            start: 418,
                                                                            end: 435,
                                                                            as_str(): "(1, Option::None)",
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0e6142520,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                                            ),
                                                                            start: 435,
                                                                            end: 436,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0e6142520,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                                        ),
                                                        start: 317,
                                                        end: 442,
                                                        as_str(): "{\n        storage.game_boards.insert(1, Option::None::<Identity>);\n        storage.game_boards.insert(1, Option::None);\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0e6142520,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR75D5Je/generics_in_contract/src/main.sw",
                                        ),
                                        start: 275,
                                        end: 444,
                                        as_str(): "{\n    #[storage(write)]\n    fn new_game() {\n        storage.game_boards.insert(1, Option::None::<Identity>);\n        storage.game_boards.insert(1, Option::None);\n    }\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [],
        },
    },
)
