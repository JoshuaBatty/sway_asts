Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe05c5b18a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe05c5b18a0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
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
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 19,
                                        as_str(): "Foo",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: Some(
                                            Annotated {
                                                attribute_list: [],
                                                value: TypeField {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 26,
                                                            end: 31,
                                                            as_str(): "value",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    colon_token: ColonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 31,
                                                            end: 32,
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
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 33,
                                                                        end: 36,
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
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 20,
                                        end: 38,
                                        as_str(): "{\n    value: u64\n}",
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
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 40,
                                        end: 44,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: None,
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe05c5b18a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                    ),
                                                    start: 45,
                                                    end: 48,
                                                    as_str(): "Foo",
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
                                                    visibility: Some(
                                                        PubToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 55,
                                                                end: 58,
                                                                as_str(): "pub",
                                                            },
                                                        },
                                                    ),
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 59,
                                                            end: 61,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 62,
                                                            end: 65,
                                                            as_str(): "set",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 74,
                                                                    end: 78,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 66,
                                                                        end: 69,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable_self: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 70,
                                                                        end: 73,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05c5b18a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                            ),
                                                                            start: 78,
                                                                            end: 79,
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
                                                                                            src (ptr): 0x00007fe05c5b18a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                            ),
                                                                                            start: 80,
                                                                                            end: 85,
                                                                                            as_str(): "value",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 85,
                                                                                        end: 86,
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
                                                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                                    ),
                                                                                                    start: 87,
                                                                                                    end: 90,
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
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 65,
                                                            end: 91,
                                                            as_str(): "(ref mut self, value: u64)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 92,
                                                                    end: 94,
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
                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                ),
                                                                                start: 95,
                                                                                end: 98,
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
                                                            Expr {
                                                                expr: Reassignment {
                                                                    assignable: FieldProjection {
                                                                        target: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 109,
                                                                                    end: 113,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                ),
                                                                                start: 113,
                                                                                end: 114,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                ),
                                                                                start: 114,
                                                                                end: 119,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    reassignment_op: ReassignmentOp {
                                                                        variant: Equals,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05c5b18a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                            ),
                                                                            start: 120,
                                                                            end: 121,
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
                                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 122,
                                                                                        end: 127,
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
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe05c5b18a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                            ),
                                                                            start: 127,
                                                                            end: 128,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                        final_expr_opt: Some(
                                                            FieldProjection {
                                                                target: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 137,
                                                                                    end: 141,
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
                                                                dot_token: DotToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 141,
                                                                        end: 142,
                                                                        as_str(): ".",
                                                                    },
                                                                },
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 142,
                                                                        end: 147,
                                                                        as_str(): "value",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5b18a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                        ),
                                                        start: 99,
                                                        end: 153,
                                                        as_str(): "{\n        self.value = value;\n        self.value\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 49,
                                        end: 155,
                                        as_str(): "{\n    pub fn set(ref mut self, value: u64) -> u64 {\n        self.value = value;\n        self.value\n    }\n}",
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
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 157,
                                            end: 159,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 160,
                                            end: 167,
                                            as_str(): "mut_foo",
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
                                                            reference: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 168,
                                                                        end: 171,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 172,
                                                                        end: 175,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 176,
                                                                    end: 179,
                                                                    as_str(): "foo",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 179,
                                                                end: 180,
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
                                                                            src (ptr): 0x00007fe05c5b18a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                            ),
                                                                            start: 181,
                                                                            end: 184,
                                                                            as_str(): "Foo",
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
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 167,
                                            end: 185,
                                            as_str(): "(ref mut foo: Foo)",
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
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 192,
                                                                        end: 195,
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
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 195,
                                                            end: 196,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 196,
                                                                end: 199,
                                                                as_str(): "set",
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
                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                ),
                                                                                start: 200,
                                                                                end: 202,
                                                                                as_str(): "10",
                                                                            },
                                                                            parsed: 10,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 203,
                                                            as_str(): "(10)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 203,
                                                            end: 204,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 186,
                                        end: 206,
                                        as_str(): "{\n    foo.set(10);\n}",
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
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 208,
                                            end: 210,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 211,
                                            end: 215,
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
                                            src (ptr): 0x00007fe05c5b18a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                            ),
                                            start: 215,
                                            end: 217,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe05c5b18a0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                    ),
                                                    start: 218,
                                                    end: 220,
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
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 221,
                                                                end: 224,
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
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 231,
                                                            end: 234,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 235,
                                                                    end: 238,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 239,
                                                                end: 242,
                                                                as_str(): "foo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 243,
                                                            end: 244,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Struct {
                                                        path: PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 245,
                                                                        end: 248,
                                                                        as_str(): "Foo",
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
                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                ),
                                                                                start: 251,
                                                                                end: 256,
                                                                                as_str(): "value",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        expr_opt: Some(
                                                                            (
                                                                                ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                        ),
                                                                                        start: 256,
                                                                                        end: 257,
                                                                                        as_str(): ":",
                                                                                    },
                                                                                },
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                                ),
                                                                                                start: 258,
                                                                                                end: 259,
                                                                                                as_str(): "0",
                                                                                            },
                                                                                            parsed: 0,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe05c5b18a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                ),
                                                                start: 249,
                                                                end: 261,
                                                                as_str(): "{ value: 0 }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 261,
                                                            end: 262,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe05c5b18a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                        ),
                                                                        start: 267,
                                                                        end: 274,
                                                                        as_str(): "mut_foo",
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
                                                            final_value_opt: Some(
                                                                Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                                    ),
                                                                                    start: 275,
                                                                                    end: 278,
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 279,
                                                            as_str(): "(foo)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe05c5b18a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                            ),
                                                            start: 279,
                                                            end: 280,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            FieldProjection {
                                                target: Path(
                                                    PathExpr {
                                                        root_opt: None,
                                                        prefix: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe05c5b18a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                                    ),
                                                                    start: 285,
                                                                    end: 288,
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
                                                dot_token: DotToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5b18a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                        ),
                                                        start: 288,
                                                        end: 289,
                                                        as_str(): ".",
                                                    },
                                                },
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe05c5b18a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                                        ),
                                                        start: 289,
                                                        end: 294,
                                                        as_str(): "value",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                            },
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe05c5b18a0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRlkiv8M/ref_mutable_fn_args_struct/src/main.sw",
                                        ),
                                        start: 225,
                                        end: 296,
                                        as_str(): "{\n    let mut foo = Foo { value: 0 };\n    mut_foo(foo);\n    foo.value\n}",
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
