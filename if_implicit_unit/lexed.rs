Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0fc1c14b0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 16,
                                            as_str(): "main",
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
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 19,
                                        end: 21,
                                        as_str(): "{}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 23,
                                            end: 26,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 27,
                                        end: 32,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 33,
                                        end: 42,
                                        as_str(): "Shiftable",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 49,
                                                            end: 51,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 52,
                                                            end: 55,
                                                            as_str(): "lsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 56,
                                                                    end: 60,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 60,
                                                                            end: 61,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
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
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 62,
                                                                                            end: 67,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 67,
                                                                                        end: 68,
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
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 69,
                                                                                                    end: 73,
                                                                                                    as_str(): "Self",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 55,
                                                            end: 74,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 75,
                                                                    end: 77,
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
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 78,
                                                                                end: 82,
                                                                                as_str(): "Self",
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
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 82,
                                                    end: 83,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 88,
                                                            end: 90,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 91,
                                                            end: 94,
                                                            as_str(): "rsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 95,
                                                                    end: 99,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 99,
                                                                            end: 100,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
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
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 101,
                                                                                            end: 106,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 106,
                                                                                        end: 107,
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
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 108,
                                                                                                    end: 112,
                                                                                                    as_str(): "Self",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 94,
                                                            end: 113,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 114,
                                                                    end: 116,
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
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 117,
                                                                                end: 121,
                                                                                as_str(): "Self",
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
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 121,
                                                    end: 122,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 124,
                                        as_str(): "{\n    fn lsh(self, other: Self) -> Self;\n    fn rsh(self, other: Self) -> Self;\n}",
                                    },
                                },
                                trait_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 126,
                                        end: 130,
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
                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                        ),
                                                        start: 131,
                                                        end: 140,
                                                        as_str(): "Shiftable",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0fc1c14b0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                ),
                                                start: 141,
                                                end: 144,
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
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 145,
                                                    end: 148,
                                                    as_str(): "u64",
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
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 155,
                                                            end: 157,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 161,
                                                            as_str(): "lsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 162,
                                                                    end: 166,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 166,
                                                                            end: 167,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
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
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 168,
                                                                                            end: 173,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 173,
                                                                                        end: 174,
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
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 175,
                                                                                                    end: 178,
                                                                                                    as_str(): "u64",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 161,
                                                            end: 179,
                                                            as_str(): "(self, other: u64)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 180,
                                                                    end: 182,
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
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 183,
                                                                                end: 187,
                                                                                as_str(): "Self",
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
                                                            Asm(
                                                                AsmBlock {
                                                                    asm_token: AsmToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 198,
                                                                            end: 201,
                                                                            as_str(): "asm",
                                                                        },
                                                                    },
                                                                    registers: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 202,
                                                                                                end: 204,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 205,
                                                                                                        end: 206,
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
                                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 207,
                                                                                                                    end: 211,
                                                                                                                    as_str(): "self",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                        incomplete_suffix: false,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 211,
                                                                                            end: 212,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 213,
                                                                                                end: 215,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 215,
                                                                                                        end: 216,
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
                                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 217,
                                                                                                                    end: 222,
                                                                                                                    as_str(): "other",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                        incomplete_suffix: false,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 222,
                                                                                            end: 223,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                AsmRegisterDeclaration {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 224,
                                                                                            end: 226,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 201,
                                                                            end: 227,
                                                                            as_str(): "(r1 : self, r2: other, r3)",
                                                                        },
                                                                    },
                                                                    contents: Braces {
                                                                        inner: AsmBlockContents {
                                                                            instructions: [
                                                                                (
                                                                                    Sll {
                                                                                        token: SllOpcode {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 242,
                                                                                                end: 245,
                                                                                                as_str(): "sll",
                                                                                            },
                                                                                        },
                                                                                        ret: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 246,
                                                                                                end: 248,
                                                                                                as_str(): "r3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        lhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 249,
                                                                                                end: 251,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        rhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 252,
                                                                                                end: 254,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 254,
                                                                                            end: 255,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_expr_opt: Some(
                                                                                AsmFinalExpr {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 268,
                                                                                            end: 270,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 270,
                                                                                                    end: 271,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                ),
                                                                                                                start: 272,
                                                                                                                end: 275,
                                                                                                                as_str(): "u64",
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
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 228,
                                                                            end: 285,
                                                                            as_str(): "{\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                        ),
                                                        start: 188,
                                                        end: 291,
                                                        as_str(): "{\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 297,
                                                            end: 299,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 300,
                                                            end: 303,
                                                            as_str(): "rsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 304,
                                                                    end: 308,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 308,
                                                                            end: 309,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
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
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 310,
                                                                                            end: 315,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 315,
                                                                                        end: 316,
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
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 317,
                                                                                                    end: 320,
                                                                                                    as_str(): "u64",
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 303,
                                                            end: 321,
                                                            as_str(): "(self, other: u64)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 322,
                                                                    end: 324,
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
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 325,
                                                                                end: 329,
                                                                                as_str(): "Self",
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
                                                            Asm(
                                                                AsmBlock {
                                                                    asm_token: AsmToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 340,
                                                                            end: 343,
                                                                            as_str(): "asm",
                                                                        },
                                                                    },
                                                                    registers: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 344,
                                                                                                end: 346,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 347,
                                                                                                        end: 348,
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
                                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 349,
                                                                                                                    end: 353,
                                                                                                                    as_str(): "self",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                        incomplete_suffix: false,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 353,
                                                                                            end: 354,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 355,
                                                                                                end: 357,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                        ),
                                                                                                        start: 357,
                                                                                                        end: 358,
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
                                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 359,
                                                                                                                    end: 364,
                                                                                                                    as_str(): "other",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            generics_opt: None,
                                                                                                        },
                                                                                                        suffix: [],
                                                                                                        incomplete_suffix: false,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 364,
                                                                                            end: 365,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: Some(
                                                                                AsmRegisterDeclaration {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 366,
                                                                                            end: 368,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 343,
                                                                            end: 369,
                                                                            as_str(): "(r1 : self, r2: other, r3)",
                                                                        },
                                                                    },
                                                                    contents: Braces {
                                                                        inner: AsmBlockContents {
                                                                            instructions: [
                                                                                (
                                                                                    Srl {
                                                                                        token: SrlOpcode {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 384,
                                                                                                end: 387,
                                                                                                as_str(): "srl",
                                                                                            },
                                                                                        },
                                                                                        ret: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 388,
                                                                                                end: 390,
                                                                                                as_str(): "r3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        lhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 391,
                                                                                                end: 393,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        rhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 394,
                                                                                                end: 396,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 396,
                                                                                            end: 397,
                                                                                            as_str(): ";",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_expr_opt: Some(
                                                                                AsmFinalExpr {
                                                                                    register: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 410,
                                                                                            end: 412,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 412,
                                                                                                    end: 413,
                                                                                                    as_str(): ":",
                                                                                                },
                                                                                            },
                                                                                            Path(
                                                                                                PathType {
                                                                                                    root_opt: None,
                                                                                                    prefix: PathTypeSegment {
                                                                                                        name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                                ),
                                                                                                                start: 414,
                                                                                                                end: 417,
                                                                                                                as_str(): "u64",
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
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 370,
                                                                            end: 427,
                                                                            as_str(): "{\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                        ),
                                                        start: 330,
                                                        end: 433,
                                                        as_str(): "{\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 149,
                                        end: 435,
                                        as_str(): "{\n    fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }\n\n    fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }\n}",
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
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 437,
                                            end: 439,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 440,
                                            end: 444,
                                            as_str(): "sqrt",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 445,
                                                                        end: 449,
                                                                        as_str(): "gas_",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 449,
                                                                    end: 450,
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
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 451,
                                                                                end: 454,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 454,
                                                                end: 455,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 456,
                                                                        end: 463,
                                                                        as_str(): "amount_",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 463,
                                                                    end: 464,
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
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 465,
                                                                                end: 468,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 468,
                                                                end: 469,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                    (
                                                        FnArg {
                                                            pattern: Var {
                                                                reference: None,
                                                                mutable: None,
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 470,
                                                                        end: 475,
                                                                        as_str(): "coin_",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                            colon_token: ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 475,
                                                                    end: 476,
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
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 477,
                                                                                end: 481,
                                                                                as_str(): "b256",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 481,
                                                                end: 482,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    FnArg {
                                                        pattern: Var {
                                                            reference: None,
                                                            mutable: None,
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 483,
                                                                    end: 488,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 488,
                                                                end: 489,
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
                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                            ),
                                                                            start: 490,
                                                                            end: 493,
                                                                            as_str(): "u64",
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
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 444,
                                            end: 494,
                                            as_str(): "(gas_: u64, amount_: u64, coin_: b256, value: u64)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 494,
                                                    end: 496,
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
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 497,
                                                                end: 500,
                                                                as_str(): "u64",
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
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 512,
                                                            end: 515,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 516,
                                                                    end: 519,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 520,
                                                                end: 521,
                                                                as_str(): "z",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 521,
                                                                    end: 522,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 522,
                                                                                end: 525,
                                                                                as_str(): "u64",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 526,
                                                            end: 527,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 528,
                                                                    end: 529,
                                                                    as_str(): "1",
                                                                },
                                                                parsed: 1,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 529,
                                                            end: 530,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 539,
                                                            end: 542,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 543,
                                                                    end: 546,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 547,
                                                                end: 548,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 548,
                                                                    end: 549,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 549,
                                                                                end: 552,
                                                                                as_str(): "u64",
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 553,
                                                            end: 554,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 555,
                                                                        end: 560,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 560,
                                                            end: 561,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 570,
                                                            end: 573,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 574,
                                                                end: 575,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 576,
                                                            end: 577,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Bool(
                                                            LitBool {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                    ),
                                                                    start: 578,
                                                                    end: 582,
                                                                    as_str(): "true",
                                                                },
                                                                kind: True,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 582,
                                                            end: 583,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: If(
                                                    IfExpr {
                                                        if_token: IfToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 592,
                                                                end: 594,
                                                                as_str(): "if",
                                                            },
                                                        },
                                                        condition: Expr(
                                                            Path(
                                                                PathExpr {
                                                                    root_opt: None,
                                                                    prefix: PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 595,
                                                                                end: 596,
                                                                                as_str(): "x",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                    incomplete_suffix: false,
                                                                },
                                                            ),
                                                        ),
                                                        then_block: Braces {
                                                            inner: CodeBlockContents {
                                                                statements: [
                                                                    Expr {
                                                                        expr: Reassignment {
                                                                            assignable: Var(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 611,
                                                                                        end: 612,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            reassignment_op: ReassignmentOp {
                                                                                variant: Equals,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 613,
                                                                                    end: 614,
                                                                                    as_str(): "=",
                                                                                },
                                                                            },
                                                                            expr: MethodCall {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 615,
                                                                                                    end: 616,
                                                                                                    as_str(): "y",
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
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 616,
                                                                                        end: 617,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                path_seg: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 617,
                                                                                            end: 620,
                                                                                            as_str(): "rsh",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                contract_args_opt: None,
                                                                                args: Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                            ),
                                                                                                            start: 621,
                                                                                                            end: 623,
                                                                                                            as_str(): "32",
                                                                                                        },
                                                                                                        parsed: 32,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 620,
                                                                                        end: 624,
                                                                                        as_str(): "(32)",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 624,
                                                                                    end: 625,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    Expr {
                                                                        expr: Reassignment {
                                                                            assignable: Var(
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 638,
                                                                                        end: 639,
                                                                                        as_str(): "z",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            reassignment_op: ReassignmentOp {
                                                                                variant: Equals,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 640,
                                                                                    end: 641,
                                                                                    as_str(): "=",
                                                                                },
                                                                            },
                                                                            expr: MethodCall {
                                                                                target: Path(
                                                                                    PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                    ),
                                                                                                    start: 642,
                                                                                                    end: 643,
                                                                                                    as_str(): "z",
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
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 643,
                                                                                        end: 644,
                                                                                        as_str(): ".",
                                                                                    },
                                                                                },
                                                                                path_seg: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 644,
                                                                                            end: 647,
                                                                                            as_str(): "lsh",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                contract_args_opt: None,
                                                                                args: Parens {
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
                                                                                            Literal(
                                                                                                Int(
                                                                                                    LitInt {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                            ),
                                                                                                            start: 648,
                                                                                                            end: 650,
                                                                                                            as_str(): "16",
                                                                                                        },
                                                                                                        parsed: 16,
                                                                                                        ty_opt: None,
                                                                                                    },
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 647,
                                                                                        end: 651,
                                                                                        as_str(): "(16)",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                        semicolon_token_opt: Some(
                                                                            SemicolonToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 651,
                                                                                    end: 652,
                                                                                    as_str(): ";",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                                final_expr_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 597,
                                                                end: 662,
                                                                as_str(): "{\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                                                            },
                                                        },
                                                        else_opt: None,
                                                    },
                                                ),
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 662,
                                                            end: 663,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Path(
                                                PathExpr {
                                                    root_opt: None,
                                                    prefix: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                ),
                                                                start: 672,
                                                                end: 673,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    suffix: [],
                                                    incomplete_suffix: false,
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 502,
                                        end: 675,
                                        as_str(): "{\n        let mut z:u64 = 1;\n        let mut y:u64 = value;\n        let x = true;\n        if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        };\n        y\n}",
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
