Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe043ecf990,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 132,
                                            end: 135,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 136,
                                        end: 141,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 142,
                                        end: 151,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 158,
                                                            end: 160,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 161,
                                                            end: 164,
                                                            as_str(): "lsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 165,
                                                                    end: 169,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 169,
                                                                            end: 170,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 171,
                                                                                            end: 176,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                        ),
                                                                                        start: 176,
                                                                                        end: 177,
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
                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                    ),
                                                                                                    start: 178,
                                                                                                    end: 181,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 164,
                                                            end: 182,
                                                            as_str(): "(self, other: u64)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 183,
                                                                    end: 185,
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
                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                ),
                                                                                start: 186,
                                                                                end: 190,
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
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 190,
                                                    end: 191,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 196,
                                                            end: 198,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 202,
                                                            as_str(): "rsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 203,
                                                                    end: 207,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 207,
                                                                            end: 208,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 209,
                                                                                            end: 214,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                        ),
                                                                                        start: 214,
                                                                                        end: 215,
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
                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                    ),
                                                                                                    start: 216,
                                                                                                    end: 219,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 202,
                                                            end: 220,
                                                            as_str(): "(self, other: u64)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 221,
                                                                    end: 223,
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
                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                ),
                                                                                start: 224,
                                                                                end: 228,
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
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 228,
                                                    end: 229,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 152,
                                        end: 232,
                                        as_str(): "{\n    fn lsh(self, other: u64) -> Self;\n    fn rsh(self, other: u64) -> Self;\n\n}",
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 234,
                                        end: 238,
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
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 239,
                                                        end: 248,
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
                                                src (ptr): 0x00007fe043ecf990,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                ),
                                                start: 249,
                                                end: 252,
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
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 253,
                                                    end: 256,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 263,
                                                            end: 265,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 269,
                                                            as_str(): "lsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 270,
                                                                    end: 274,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 274,
                                                                            end: 275,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 276,
                                                                                            end: 281,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                        ),
                                                                                        start: 281,
                                                                                        end: 282,
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
                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                    ),
                                                                                                    start: 283,
                                                                                                    end: 286,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 269,
                                                            end: 287,
                                                            as_str(): "(self, other: u64)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 288,
                                                                    end: 290,
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
                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                ),
                                                                                start: 291,
                                                                                end: 295,
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
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 306,
                                                                            end: 309,
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
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 310,
                                                                                                end: 312,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                        ),
                                                                                                        start: 313,
                                                                                                        end: 314,
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
                                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 315,
                                                                                                                    end: 319,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 319,
                                                                                            end: 320,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 321,
                                                                                                end: 323,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                        ),
                                                                                                        start: 323,
                                                                                                        end: 324,
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
                                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 325,
                                                                                                                    end: 330,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 330,
                                                                                            end: 331,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 332,
                                                                                            end: 334,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 309,
                                                                            end: 335,
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
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 350,
                                                                                                end: 353,
                                                                                                as_str(): "sll",
                                                                                            },
                                                                                        },
                                                                                        ret: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 354,
                                                                                                end: 356,
                                                                                                as_str(): "r3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        lhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 357,
                                                                                                end: 359,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        rhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 360,
                                                                                                end: 362,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 362,
                                                                                            end: 363,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 376,
                                                                                            end: 378,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                    ),
                                                                                                    start: 378,
                                                                                                    end: 379,
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
                                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                                ),
                                                                                                                start: 380,
                                                                                                                end: 383,
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
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 336,
                                                                            end: 393,
                                                                            as_str(): "{\n            sll r3 r1 r2;\n            r3: u64\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 296,
                                                        end: 399,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 405,
                                                            end: 407,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 408,
                                                            end: 411,
                                                            as_str(): "rsh",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 412,
                                                                    end: 416,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 416,
                                                                            end: 417,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 418,
                                                                                            end: 423,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                        ),
                                                                                        start: 423,
                                                                                        end: 424,
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
                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                    ),
                                                                                                    start: 425,
                                                                                                    end: 428,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 411,
                                                            end: 429,
                                                            as_str(): "(self, other: u64)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 430,
                                                                    end: 432,
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
                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                ),
                                                                                start: 433,
                                                                                end: 437,
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
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 448,
                                                                            end: 451,
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
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 452,
                                                                                                end: 454,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                        ),
                                                                                                        start: 455,
                                                                                                        end: 456,
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
                                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 457,
                                                                                                                    end: 461,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 461,
                                                                                            end: 462,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    AsmRegisterDeclaration {
                                                                                        register: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 463,
                                                                                                end: 465,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        value_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe043ecf990,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                        ),
                                                                                                        start: 465,
                                                                                                        end: 466,
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
                                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 467,
                                                                                                                    end: 472,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 472,
                                                                                            end: 473,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 474,
                                                                                            end: 476,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    value_opt: None,
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 451,
                                                                            end: 477,
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
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 492,
                                                                                                end: 495,
                                                                                                as_str(): "srl",
                                                                                            },
                                                                                        },
                                                                                        ret: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 496,
                                                                                                end: 498,
                                                                                                as_str(): "r3",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        lhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 499,
                                                                                                end: 501,
                                                                                                as_str(): "r1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        rhs: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                ),
                                                                                                start: 502,
                                                                                                end: 504,
                                                                                                as_str(): "r2",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    SemicolonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 504,
                                                                                            end: 505,
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
                                                                                            src (ptr): 0x00007fe043ecf990,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                            ),
                                                                                            start: 518,
                                                                                            end: 520,
                                                                                            as_str(): "r3",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            ColonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe043ecf990,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                                    ),
                                                                                                    start: 520,
                                                                                                    end: 521,
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
                                                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
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
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe043ecf990,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                            ),
                                                                            start: 478,
                                                                            end: 535,
                                                                            as_str(): "{\n            srl r3 r1 r2;\n            r3: u64\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 438,
                                                        end: 541,
                                                        as_str(): "{\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 257,
                                        end: 543,
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
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 545,
                                            end: 547,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 548,
                                            end: 551,
                                            as_str(): "foo",
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
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 551,
                                            end: 553,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 554,
                                                    end: 556,
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
                                                                src (ptr): 0x00007fe043ecf990,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                ),
                                                                start: 557,
                                                                end: 560,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 567,
                                                            end: 570,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 571,
                                                                    end: 574,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe043ecf990,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                ),
                                                                start: 575,
                                                                end: 576,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 576,
                                                                    end: 577,
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
                                                                                src (ptr): 0x00007fe043ecf990,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                                ),
                                                                                start: 578,
                                                                                end: 581,
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
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 582,
                                                            end: 583,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 584,
                                                                    end: 585,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 585,
                                                            end: 586,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Reassignment {
                                                    assignable: Var(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe043ecf990,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                ),
                                                                start: 591,
                                                                end: 592,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 593,
                                                            end: 594,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Add {
                                                        lhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 595,
                                                                        end: 596,
                                                                        as_str(): "5",
                                                                    },
                                                                    parsed: 5,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                        add_token: AddToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe043ecf990,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                ),
                                                                start: 597,
                                                                end: 598,
                                                                as_str(): "+",
                                                            },
                                                        },
                                                        rhs: Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe043ecf990,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                        ),
                                                                        start: 599,
                                                                        end: 600,
                                                                        as_str(): "2",
                                                                    },
                                                                    parsed: 2,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe043ecf990,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                            ),
                                                            start: 600,
                                                            end: 601,
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
                                                                src (ptr): 0x00007fe043ecf990,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                ),
                                                                start: 606,
                                                                end: 607,
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
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 561,
                                        end: 609,
                                        as_str(): "{\n    let mut x: u64 = 4;\n    x = 5 + 2;\n    x\n}",
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
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 611,
                                            end: 613,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 614,
                                            end: 618,
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
                                            src (ptr): 0x00007fe043ecf990,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                            ),
                                            start: 618,
                                            end: 620,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 621,
                                                    end: 623,
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
                                                                src (ptr): 0x00007fe043ecf990,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                ),
                                                                start: 624,
                                                                end: 627,
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
                                        statements: [],
                                        final_expr_opt: Some(
                                            FuncApp {
                                                func: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe043ecf990,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                                    ),
                                                                    start: 632,
                                                                    end: 635,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            generics_opt: None,
                                                        },
                                                        suffix: [],
                                                        incomplete_suffix: false,
                                                    },
                                                ),
                                                args: Parens {
                                                    inner: Punctuated {
                                                        value_separator_pairs: [],
                                                        final_value_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 635,
                                                        end: 637,
                                                        as_str(): "()",
                                                    },
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 628,
                                        end: 639,
                                        as_str(): "{\n  foo()\n}",
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
