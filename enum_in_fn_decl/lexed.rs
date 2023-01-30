Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0fa919aa0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0fa919aa0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
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
                                            src (ptr): 0x00007fb0fa919aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                            ),
                                            start: 8,
                                            end: 10,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0fa919aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                            ),
                                            start: 11,
                                            end: 15,
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
                                            src (ptr): 0x00007fb0fa919aa0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                            ),
                                            start: 15,
                                            end: 17,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0fa919aa0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                    ),
                                                    start: 18,
                                                    end: 20,
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
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 21,
                                                                end: 24,
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
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 31,
                                                            end: 34,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 36,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 37,
                                                            end: 38,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 39,
                                                                    end: 42,
                                                                    as_str(): "255",
                                                                },
                                                                parsed: 255,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 42,
                                                            end: 43,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Enum(
                                                        ItemEnum {
                                                            visibility: None,
                                                            enum_token: EnumToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 49,
                                                                    end: 53,
                                                                    as_str(): "enum",
                                                                },
                                                            },
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 54,
                                                                    end: 55,
                                                                    as_str(): "X",
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
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 66,
                                                                                            end: 67,
                                                                                            as_str(): "Y",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    colon_token: ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
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
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 69,
                                                                                                        end: 73,
                                                                                                        as_str(): "bool",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                generics_opt: None,
                                                                                            },
                                                                                            suffix: [],
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            },
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 73,
                                                                                    end: 74,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    final_value_opt: None,
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 56,
                                                                    end: 80,
                                                                    as_str(): "{\n        Y: bool,\n    }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Impl(
                                                        ItemImpl {
                                                            impl_token: ImplToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 86,
                                                                    end: 90,
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
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 91,
                                                                                    end: 95,
                                                                                    as_str(): "core",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 95,
                                                                                        end: 97,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 97,
                                                                                            end: 100,
                                                                                            as_str(): "ops",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 100,
                                                                                        end: 102,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 102,
                                                                                            end: 104,
                                                                                            as_str(): "Eq",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    ForToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                            ),
                                                                            start: 105,
                                                                            end: 108,
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
                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                ),
                                                                                start: 109,
                                                                                end: 110,
                                                                                as_str(): "X",
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
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 121,
                                                                                        end: 123,
                                                                                        as_str(): "fn",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 124,
                                                                                        end: 126,
                                                                                        as_str(): "eq",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics: None,
                                                                                arguments: Parens {
                                                                                    inner: NonStatic {
                                                                                        self_token: SelfToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 127,
                                                                                                end: 131,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                        ref_self: None,
                                                                                        mutable_self: None,
                                                                                        args_opt: Some(
                                                                                            (
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 131,
                                                                                                        end: 132,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 133,
                                                                                                                        end: 138,
                                                                                                                        as_str(): "other",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                            colon_token: ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 138,
                                                                                                                    end: 139,
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
                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 140,
                                                                                                                                end: 144,
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
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 126,
                                                                                        end: 145,
                                                                                        as_str(): "(self, other: Self)",
                                                                                    },
                                                                                },
                                                                                return_type_opt: Some(
                                                                                    (
                                                                                        RightArrowToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 146,
                                                                                                end: 148,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 149,
                                                                                                            end: 153,
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
                                                                                        Asm(
                                                                                            AsmBlock {
                                                                                                asm_token: AsmToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 168,
                                                                                                        end: 171,
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
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 172,
                                                                                                                            end: 174,
                                                                                                                            as_str(): "r1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 174,
                                                                                                                                    end: 175,
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
                                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 176,
                                                                                                                                                end: 180,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 180,
                                                                                                                        end: 181,
                                                                                                                        as_str(): ",",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            (
                                                                                                                AsmRegisterDeclaration {
                                                                                                                    register: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 182,
                                                                                                                            end: 184,
                                                                                                                            as_str(): "r2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 184,
                                                                                                                                    end: 185,
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
                                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 186,
                                                                                                                                                end: 191,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 191,
                                                                                                                        end: 192,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 193,
                                                                                                                        end: 195,
                                                                                                                        as_str(): "r3",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                value_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 171,
                                                                                                        end: 196,
                                                                                                        as_str(): "(r1: self, r2: other, r3)",
                                                                                                    },
                                                                                                },
                                                                                                contents: Braces {
                                                                                                    inner: AsmBlockContents {
                                                                                                        instructions: [
                                                                                                            (
                                                                                                                Eq {
                                                                                                                    token: EqOpcode {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 215,
                                                                                                                            end: 217,
                                                                                                                            as_str(): "eq",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    ret: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 218,
                                                                                                                            end: 220,
                                                                                                                            as_str(): "r3",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    lhs: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 221,
                                                                                                                            end: 223,
                                                                                                                            as_str(): "r2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    rhs: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 224,
                                                                                                                            end: 226,
                                                                                                                            as_str(): "r1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                                SemicolonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 226,
                                                                                                                        end: 227,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 244,
                                                                                                                        end: 246,
                                                                                                                        as_str(): "r3",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                ty_opt: Some(
                                                                                                                    (
                                                                                                                        ColonToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 246,
                                                                                                                                end: 247,
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
                                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 248,
                                                                                                                                            end: 252,
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
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 197,
                                                                                                        end: 266,
                                                                                                        as_str(): "{\n                eq r3 r2 r1;\n                r3: bool\n            }",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 154,
                                                                                    end: 276,
                                                                                    as_str(): "{\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 111,
                                                                    end: 282,
                                                                    as_str(): "{\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                            Item(
                                                Annotated {
                                                    attribute_list: [],
                                                    value: Impl(
                                                        ItemImpl {
                                                            impl_token: ImplToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 288,
                                                                    end: 292,
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
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 293,
                                                                                    end: 297,
                                                                                    as_str(): "core",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 297,
                                                                                        end: 299,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 299,
                                                                                            end: 302,
                                                                                            as_str(): "ops",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 302,
                                                                                        end: 304,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 304,
                                                                                            end: 307,
                                                                                            as_str(): "Ord",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    ForToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                            ),
                                                                            start: 308,
                                                                            end: 311,
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
                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                ),
                                                                                start: 312,
                                                                                end: 313,
                                                                                as_str(): "X",
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
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 324,
                                                                                        end: 326,
                                                                                        as_str(): "fn",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 327,
                                                                                        end: 329,
                                                                                        as_str(): "lt",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics: None,
                                                                                arguments: Parens {
                                                                                    inner: NonStatic {
                                                                                        self_token: SelfToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 330,
                                                                                                end: 334,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                        ref_self: None,
                                                                                        mutable_self: None,
                                                                                        args_opt: Some(
                                                                                            (
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 334,
                                                                                                        end: 335,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 336,
                                                                                                                        end: 341,
                                                                                                                        as_str(): "other",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                            colon_token: ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 341,
                                                                                                                    end: 342,
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
                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 343,
                                                                                                                                end: 347,
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
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 329,
                                                                                        end: 348,
                                                                                        as_str(): "(self, other: Self)",
                                                                                    },
                                                                                },
                                                                                return_type_opt: Some(
                                                                                    (
                                                                                        RightArrowToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 349,
                                                                                                end: 351,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 352,
                                                                                                            end: 356,
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
                                                                                        Asm(
                                                                                            AsmBlock {
                                                                                                asm_token: AsmToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 371,
                                                                                                        end: 374,
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
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 375,
                                                                                                                            end: 377,
                                                                                                                            as_str(): "r1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 377,
                                                                                                                                    end: 378,
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
                                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 379,
                                                                                                                                                end: 383,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 383,
                                                                                                                        end: 384,
                                                                                                                        as_str(): ",",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            (
                                                                                                                AsmRegisterDeclaration {
                                                                                                                    register: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 385,
                                                                                                                            end: 387,
                                                                                                                            as_str(): "r2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 387,
                                                                                                                                    end: 388,
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
                                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 389,
                                                                                                                                                end: 394,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 394,
                                                                                                                        end: 395,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 396,
                                                                                                                        end: 398,
                                                                                                                        as_str(): "r3",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                value_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 374,
                                                                                                        end: 399,
                                                                                                        as_str(): "(r1: self, r2: other, r3)",
                                                                                                    },
                                                                                                },
                                                                                                contents: Braces {
                                                                                                    inner: AsmBlockContents {
                                                                                                        instructions: [
                                                                                                            (
                                                                                                                Lt {
                                                                                                                    token: LtOpcode {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 418,
                                                                                                                            end: 420,
                                                                                                                            as_str(): "lt",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    ret: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 421,
                                                                                                                            end: 423,
                                                                                                                            as_str(): "r3",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    lhs: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 424,
                                                                                                                            end: 426,
                                                                                                                            as_str(): "r2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    rhs: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 427,
                                                                                                                            end: 429,
                                                                                                                            as_str(): "r1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                                SemicolonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 429,
                                                                                                                        end: 430,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 447,
                                                                                                                        end: 449,
                                                                                                                        as_str(): "r3",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                ty_opt: Some(
                                                                                                                    (
                                                                                                                        ColonToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 449,
                                                                                                                                end: 450,
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
                                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 451,
                                                                                                                                            end: 455,
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
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 400,
                                                                                                        end: 469,
                                                                                                        as_str(): "{\n                lt r3 r2 r1;\n                r3: bool\n            }",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 357,
                                                                                    end: 479,
                                                                                    as_str(): "{\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }",
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
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 488,
                                                                                        end: 490,
                                                                                        as_str(): "fn",
                                                                                    },
                                                                                },
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 491,
                                                                                        end: 493,
                                                                                        as_str(): "gt",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics: None,
                                                                                arguments: Parens {
                                                                                    inner: NonStatic {
                                                                                        self_token: SelfToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 494,
                                                                                                end: 498,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                        },
                                                                                        ref_self: None,
                                                                                        mutable_self: None,
                                                                                        args_opt: Some(
                                                                                            (
                                                                                                CommaToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 498,
                                                                                                        end: 499,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 500,
                                                                                                                        end: 505,
                                                                                                                        as_str(): "other",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                            colon_token: ColonToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 505,
                                                                                                                    end: 506,
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
                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 507,
                                                                                                                                end: 511,
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
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 493,
                                                                                        end: 512,
                                                                                        as_str(): "(self, other: Self)",
                                                                                    },
                                                                                },
                                                                                return_type_opt: Some(
                                                                                    (
                                                                                        RightArrowToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 513,
                                                                                                end: 515,
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
                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                            ),
                                                                                                            start: 516,
                                                                                                            end: 520,
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
                                                                                        Asm(
                                                                                            AsmBlock {
                                                                                                asm_token: AsmToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 535,
                                                                                                        end: 538,
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
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 539,
                                                                                                                            end: 541,
                                                                                                                            as_str(): "r1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 541,
                                                                                                                                    end: 542,
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
                                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 543,
                                                                                                                                                end: 547,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 547,
                                                                                                                        end: 548,
                                                                                                                        as_str(): ",",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                            (
                                                                                                                AsmRegisterDeclaration {
                                                                                                                    register: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 549,
                                                                                                                            end: 551,
                                                                                                                            as_str(): "r2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    value_opt: Some(
                                                                                                                        (
                                                                                                                            ColonToken {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 551,
                                                                                                                                    end: 552,
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
                                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                                path: Some(
                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                                ),
                                                                                                                                                start: 553,
                                                                                                                                                end: 558,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 558,
                                                                                                                        end: 559,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 560,
                                                                                                                        end: 562,
                                                                                                                        as_str(): "r3",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                value_opt: None,
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 538,
                                                                                                        end: 563,
                                                                                                        as_str(): "(r1: self, r2: other, r3)",
                                                                                                    },
                                                                                                },
                                                                                                contents: Braces {
                                                                                                    inner: AsmBlockContents {
                                                                                                        instructions: [
                                                                                                            (
                                                                                                                Gt {
                                                                                                                    token: GtOpcode {
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 582,
                                                                                                                            end: 584,
                                                                                                                            as_str(): "gt",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    ret: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 585,
                                                                                                                            end: 587,
                                                                                                                            as_str(): "r3",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    lhs: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 588,
                                                                                                                            end: 590,
                                                                                                                            as_str(): "r2",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    rhs: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 591,
                                                                                                                            end: 593,
                                                                                                                            as_str(): "r1",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                                SemicolonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 593,
                                                                                                                        end: 594,
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
                                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 611,
                                                                                                                        end: 613,
                                                                                                                        as_str(): "r3",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                ty_opt: Some(
                                                                                                                    (
                                                                                                                        ColonToken {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 613,
                                                                                                                                end: 614,
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
                                                                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 615,
                                                                                                                                            end: 619,
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
                                                                                                            },
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                        ),
                                                                                                        start: 564,
                                                                                                        end: 633,
                                                                                                        as_str(): "{\n                gt r3 r2 r1;\n                r3: bool\n            }",
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 521,
                                                                                    end: 643,
                                                                                    as_str(): "{\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 314,
                                                                    end: 649,
                                                                    as_str(): "{\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }",
                                                                },
                                                            },
                                                        },
                                                    ),
                                                },
                                            ),
                                        ],
                                        final_expr_opt: Some(
                                            If(
                                                IfExpr {
                                                    if_token: IfToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 654,
                                                            end: 656,
                                                            as_str(): "if",
                                                        },
                                                    },
                                                    condition: Expr(
                                                        Equal {
                                                            lhs: FuncApp {
                                                                func: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 657,
                                                                                    end: 658,
                                                                                    as_str(): "X",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 658,
                                                                                        end: 660,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 660,
                                                                                            end: 661,
                                                                                            as_str(): "Y",
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
                                                                args: Parens {
                                                                    inner: Punctuated {
                                                                        value_separator_pairs: [],
                                                                        final_value_opt: Some(
                                                                            Literal(
                                                                                Bool(
                                                                                    LitBool {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 662,
                                                                                            end: 666,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                        kind: True,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                        ),
                                                                        start: 661,
                                                                        end: 667,
                                                                        as_str(): "(true)",
                                                                    },
                                                                },
                                                            },
                                                            double_eq_token: DoubleEqToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 668,
                                                                    end: 670,
                                                                    as_str(): "==",
                                                                },
                                                            },
                                                            rhs: FuncApp {
                                                                func: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 671,
                                                                                    end: 672,
                                                                                    as_str(): "X",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                        ),
                                                                                        start: 672,
                                                                                        end: 674,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 674,
                                                                                            end: 675,
                                                                                            as_str(): "Y",
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
                                                                args: Parens {
                                                                    inner: Punctuated {
                                                                        value_separator_pairs: [],
                                                                        final_value_opt: Some(
                                                                            Literal(
                                                                                Bool(
                                                                                    LitBool {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0fa919aa0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                            ),
                                                                                            start: 676,
                                                                                            end: 680,
                                                                                            as_str(): "true",
                                                                                        },
                                                                                        kind: True,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                        ),
                                                                        start: 675,
                                                                        end: 681,
                                                                        as_str(): "(true)",
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    ),
                                                    then_block: Braces {
                                                        inner: CodeBlockContents {
                                                            statements: [],
                                                            final_expr_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                    ),
                                                                                    start: 692,
                                                                                    end: 693,
                                                                                    as_str(): "a",
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
                                                            src (ptr): 0x00007fb0fa919aa0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                            ),
                                                            start: 682,
                                                            end: 699,
                                                            as_str(): "{\n        a\n    }",
                                                        },
                                                    },
                                                    else_opt: Some(
                                                        (
                                                            ElseToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0fa919aa0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                    ),
                                                                    start: 700,
                                                                    end: 704,
                                                                    as_str(): "else",
                                                                },
                                                            },
                                                            Break(
                                                                Braces {
                                                                    inner: CodeBlockContents {
                                                                        statements: [],
                                                                        final_expr_opt: Some(
                                                                            Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0fa919aa0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                                                ),
                                                                                                start: 715,
                                                                                                end: 716,
                                                                                                as_str(): "a",
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
                                                                        src (ptr): 0x00007fb0fa919aa0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                                                        ),
                                                                        start: 705,
                                                                        end: 722,
                                                                        as_str(): "{\n        a\n    }",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0fa919aa0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRWBBKiW/enum_in_fn_decl/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 724,
                                        as_str(): "{\n    let a = 255;\n\n    enum X {\n        Y: bool,\n    }\n\n    impl core::ops::Eq for X {\n        fn eq(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                eq r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n\n    impl core::ops::Ord for X {\n        fn lt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                lt r3 r2 r1;\n                r3: bool\n            }\n        }\n        fn gt(self, other: Self) -> bool {\n            asm(r1: self, r2: other, r3) {\n                gt r3 r2 r1;\n                r3: bool\n            }\n        }\n    }\n    if X::Y(true) == X::Y(true) {\n        a\n    } else {\n        a\n    }\n}",
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
