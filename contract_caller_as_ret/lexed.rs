Some(
    LexedProgram {
        kind: Contract,
        root: LexedModule {
            tree: Module {
                kind: Contract {
                    contract_token: ContractToken {
                        span: Span {
                            src (ptr): 0x00007fb10915a7d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                            ),
                            start: 0,
                            end: 8,
                            as_str(): "contract",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb10915a7d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                        ),
                        start: 8,
                        end: 9,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Use(
                            ItemUse {
                                visibility: None,
                                use_token: UseToken {
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 10,
                                        end: 13,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb10915a7d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                            ),
                                            start: 14,
                                            end: 17,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fb10915a7d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                            ),
                                            start: 17,
                                            end: 19,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Path {
                                        prefix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb10915a7d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                ),
                                                start: 19,
                                                end: 30,
                                                as_str(): "contract_id",
                                            },
                                            is_raw_ident: false,
                                        },
                                        double_colon_token: DoubleColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fb10915a7d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                ),
                                                start: 30,
                                                end: 32,
                                                as_str(): "::",
                                            },
                                        },
                                        suffix: Name {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb10915a7d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                    ),
                                                    start: 32,
                                                    end: 42,
                                                    as_str(): "ContractId",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 42,
                                        end: 43,
                                        as_str(): ";",
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
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 45,
                                        end: 48,
                                        as_str(): "abi",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 49,
                                        end: 59,
                                        as_str(): "MyContract",
                                    },
                                    is_raw_ident: false,
                                },
                                abi_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10915a7d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                            ),
                                                            start: 66,
                                                            end: 68,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10915a7d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                            ),
                                                            start: 69,
                                                            end: 82,
                                                            as_str(): "test_function",
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
                                                            src (ptr): 0x00007fb10915a7d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                            ),
                                                            start: 82,
                                                            end: 84,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10915a7d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                    ),
                                                                    start: 85,
                                                                    end: 87,
                                                                    as_str(): "->",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10915a7d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                                ),
                                                                                start: 88,
                                                                                end: 92,
                                                                                as_str(): "bool",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    where_clause_opt: None,
                                                },
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb10915a7d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                    ),
                                                    start: 92,
                                                    end: 93,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 60,
                                        end: 95,
                                        as_str(): "{\n    fn test_function() -> bool;\n}",
                                    },
                                },
                                abi_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 97,
                                        end: 101,
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
                                                        src (ptr): 0x00007fb10915a7d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                        ),
                                                        start: 102,
                                                        end: 112,
                                                        as_str(): "MyContract",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fb10915a7d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                ),
                                                start: 113,
                                                end: 116,
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
                                                    src (ptr): 0x00007fb10915a7d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                    ),
                                                    start: 117,
                                                    end: 125,
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
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb10915a7d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                            ),
                                                            start: 132,
                                                            end: 134,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb10915a7d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 148,
                                                            as_str(): "test_function",
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
                                                            src (ptr): 0x00007fb10915a7d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                            ),
                                                            start: 148,
                                                            end: 150,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10915a7d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                    ),
                                                                    start: 151,
                                                                    end: 153,
                                                                    as_str(): "->",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10915a7d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                                ),
                                                                                start: 154,
                                                                                end: 158,
                                                                                as_str(): "bool",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb10915a7d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                            ),
                                                                            start: 169,
                                                                            end: 173,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb10915a7d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                        ),
                                                        start: 159,
                                                        end: 179,
                                                        as_str(): "{\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 126,
                                        end: 181,
                                        as_str(): "{\n    fn test_function() -> bool {\n        true\n    }\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fb10915a7d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                            ),
                                            start: 183,
                                            end: 185,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb10915a7d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                            ),
                                            start: 186,
                                            end: 192,
                                            as_str(): "caller",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10915a7d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                    ),
                                                                    start: 193,
                                                                    end: 200,
                                                                    as_str(): "address",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10915a7d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                ),
                                                                start: 200,
                                                                end: 201,
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
                                                                            src (ptr): 0x00007fb10915a7d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                            ),
                                                                            start: 202,
                                                                            end: 212,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [],
                                                            },
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb10915a7d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                            ),
                                            start: 192,
                                            end: 213,
                                            as_str(): "(address: ContractId)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb10915a7d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                    ),
                                                    start: 214,
                                                    end: 216,
                                                    as_str(): "->",
                                                },
                                            },
                                            Path(
                                                PathType {
                                                    root_opt: None,
                                                    prefix: PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb10915a7d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                ),
                                                                start: 217,
                                                                end: 231,
                                                                as_str(): "ContractCaller",
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
                                                                                src (ptr): 0x00007fb10915a7d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                                ),
                                                                                start: 231,
                                                                                end: 232,
                                                                                as_str(): "<",
                                                                            },
                                                                        },
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Infer {
                                                                                    underscore_token: UnderscoreToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb10915a7d0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                                            ),
                                                                                            start: 232,
                                                                                            end: 233,
                                                                                            as_str(): "_",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        close_angle_bracket_token: CloseAngleBracketToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10915a7d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                                ),
                                                                                start: 233,
                                                                                end: 234,
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
                                    ),
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: Some(
                                            AbiCast {
                                                abi_token: AbiToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb10915a7d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                        ),
                                                        start: 239,
                                                        end: 242,
                                                        as_str(): "abi",
                                                    },
                                                },
                                                args: Parens {
                                                    inner: AbiCastArgs {
                                                        name: PathType {
                                                            root_opt: None,
                                                            prefix: PathTypeSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb10915a7d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                        ),
                                                                        start: 243,
                                                                        end: 253,
                                                                        as_str(): "MyContract",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                        },
                                                        comma_token: CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb10915a7d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                ),
                                                                start: 253,
                                                                end: 254,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                        address: FieldProjection {
                                                            target: Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb10915a7d0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                                ),
                                                                                start: 255,
                                                                                end: 262,
                                                                                as_str(): "address",
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
                                                                    src (ptr): 0x00007fb10915a7d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                    ),
                                                                    start: 262,
                                                                    end: 263,
                                                                    as_str(): ".",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb10915a7d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                                    ),
                                                                    start: 263,
                                                                    end: 268,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb10915a7d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                                        ),
                                                        start: 242,
                                                        end: 269,
                                                        as_str(): "(MyContract, address.value)",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb10915a7d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqTOiKb/contract_caller_as_ret/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 271,
                                        as_str(): "{\n  abi(MyContract, address.value)\n}",
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
