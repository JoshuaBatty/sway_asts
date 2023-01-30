Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe04ae0ff10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe04ae0ff10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
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
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 12,
                                        as_str(): "use",
                                    },
                                },
                                root_import: None,
                                tree: Path {
                                    prefix: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 13,
                                            end: 16,
                                            as_str(): "std",
                                        },
                                        is_raw_ident: false,
                                    },
                                    double_colon_token: DoubleColonToken {
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 16,
                                            end: 18,
                                            as_str(): "::",
                                        },
                                    },
                                    suffix: Group {
                                        imports: Braces {
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        Path {
                                                            prefix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 19,
                                                                    end: 25,
                                                                    as_str(): "assert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            double_colon_token: DoubleColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 25,
                                                                    end: 27,
                                                                    as_str(): "::",
                                                                },
                                                            },
                                                            suffix: Name {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 27,
                                                                        end: 33,
                                                                        as_str(): "assert",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                            },
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 33,
                                                                end: 34,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    Path {
                                                        prefix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 35,
                                                                end: 41,
                                                                as_str(): "revert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        double_colon_token: DoubleColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 41,
                                                                end: 43,
                                                                as_str(): "::",
                                                            },
                                                        },
                                                        suffix: Name {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 43,
                                                                    end: 49,
                                                                    as_str(): "revert",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                    },
                                                ),
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe04ae0ff10,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                ),
                                                start: 18,
                                                end: 50,
                                                as_str(): "{assert::assert, revert::revert}",
                                            },
                                        },
                                    },
                                },
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 51,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Struct(
                            ItemStruct {
                                visibility: None,
                                struct_token: StructToken {
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 53,
                                        end: 59,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 60,
                                        end: 61,
                                        as_str(): "A",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 68,
                                                                end: 69,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 69,
                                                                end: 70,
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
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 71,
                                                                            end: 74,
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
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 74,
                                                        end: 75,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 80,
                                                                end: 81,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 81,
                                                                end: 82,
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
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 83,
                                                                            end: 86,
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
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 86,
                                                        end: 87,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 62,
                                        end: 89,
                                        as_str(): "{\n    a: u64,\n    b: u64,\n}",
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
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 91,
                                        end: 95,
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
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 96,
                                                    end: 97,
                                                    as_str(): "A",
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 104,
                                                            end: 106,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 107,
                                                            end: 108,
                                                            as_str(): "f",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 117,
                                                                    end: 121,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 109,
                                                                        end: 112,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable_self: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 113,
                                                                        end: 116,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 108,
                                                            end: 122,
                                                            as_str(): "(ref mut self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 133,
                                                                                    end: 137,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 137,
                                                                                end: 138,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 138,
                                                                                end: 139,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    reassignment_op: ReassignmentOp {
                                                                        variant: Equals,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 141,
                                                                            as_str(): "=",
                                                                        },
                                                                    },
                                                                    expr: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 142,
                                                                                    end: 144,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 144,
                                                                            end: 145,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            Expr {
                                                                expr: Reassignment {
                                                                    assignable: FieldProjection {
                                                                        target: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 154,
                                                                                    end: 158,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 158,
                                                                                end: 159,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 159,
                                                                                end: 160,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    reassignment_op: ReassignmentOp {
                                                                        variant: Equals,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 161,
                                                                            end: 162,
                                                                            as_str(): "=",
                                                                        },
                                                                    },
                                                                    expr: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 163,
                                                                                    end: 165,
                                                                                    as_str(): "77",
                                                                                },
                                                                                parsed: 77,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 165,
                                                                            end: 166,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 123,
                                                        end: 172,
                                                        as_str(): "{\n        self.a = 42;\n        self.b = 77;\n    }",
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 178,
                                                            end: 180,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 181,
                                                            end: 182,
                                                            as_str(): "g",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 191,
                                                                    end: 195,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 183,
                                                                        end: 186,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable_self: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 187,
                                                                        end: 190,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 195,
                                                                            end: 196,
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
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 197,
                                                                                            end: 200,
                                                                                            as_str(): "inc",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
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
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 202,
                                                                                                    end: 205,
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 206,
                                                            as_str(): "(ref mut self, inc: u64)",
                                                        },
                                                    },
                                                    return_type_opt: None,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 217,
                                                                                    end: 221,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 221,
                                                                                end: 222,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 222,
                                                                                end: 223,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    reassignment_op: ReassignmentOp {
                                                                        variant: Equals,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 224,
                                                                            end: 225,
                                                                            as_str(): "=",
                                                                        },
                                                                    },
                                                                    expr: Add {
                                                                        lhs: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 226,
                                                                                                end: 230,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 230,
                                                                                    end: 231,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 231,
                                                                                    end: 232,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        add_token: AddToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 233,
                                                                                end: 234,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                        rhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 235,
                                                                                            end: 238,
                                                                                            as_str(): "inc",
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
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 238,
                                                                            end: 239,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            Expr {
                                                                expr: Reassignment {
                                                                    assignable: FieldProjection {
                                                                        target: Var(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 248,
                                                                                    end: 252,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 252,
                                                                                end: 253,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 253,
                                                                                end: 254,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    reassignment_op: ReassignmentOp {
                                                                        variant: Equals,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 255,
                                                                            end: 256,
                                                                            as_str(): "=",
                                                                        },
                                                                    },
                                                                    expr: Add {
                                                                        lhs: FieldProjection {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 257,
                                                                                                end: 261,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 261,
                                                                                    end: 262,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 262,
                                                                                    end: 263,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        },
                                                                        add_token: AddToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 264,
                                                                                end: 265,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                        rhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 266,
                                                                                            end: 269,
                                                                                            as_str(): "inc",
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
                                                                },
                                                                semicolon_token_opt: Some(
                                                                    SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 269,
                                                                            end: 270,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                        ],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 207,
                                                        end: 276,
                                                        as_str(): "{\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }",
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 284,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 285,
                                                            end: 286,
                                                            as_str(): "h",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 295,
                                                                    end: 299,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 287,
                                                                        end: 290,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable_self: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 291,
                                                                        end: 294,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 286,
                                                            end: 300,
                                                            as_str(): "(ref mut self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Reassignment {
                                                                assignable: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 311,
                                                                            end: 315,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                reassignment_op: ReassignmentOp {
                                                                    variant: Equals,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 316,
                                                                        end: 317,
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
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 318,
                                                                                    end: 319,
                                                                                    as_str(): "A",
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
                                                                            value_separator_pairs: [
                                                                                (
                                                                                    ExprStructField {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 334,
                                                                                                end: 335,
                                                                                                as_str(): "a",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 335,
                                                                                                        end: 336,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 337,
                                                                                                                end: 340,
                                                                                                                as_str(): "100",
                                                                                                            },
                                                                                                            parsed: 100,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 340,
                                                                                            end: 341,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    ExprStructField {
                                                                                        field_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 354,
                                                                                                end: 355,
                                                                                                as_str(): "b",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        expr_opt: Some(
                                                                                            (
                                                                                                ColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 355,
                                                                                                        end: 356,
                                                                                                        as_str(): ":",
                                                                                                    },
                                                                                                },
                                                                                                Literal(
                                                                                                    Int(
                                                                                                        LitInt {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 357,
                                                                                                                end: 360,
                                                                                                                as_str(): "200",
                                                                                                            },
                                                                                                            parsed: 200,
                                                                                                            ty_opt: None,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 360,
                                                                                            end: 361,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            final_value_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 320,
                                                                            end: 371,
                                                                            as_str(): "{\n            a: 100,\n            b: 200,\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 301,
                                                        end: 377,
                                                        as_str(): "{\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 379,
                                        as_str(): "{\n    fn f(ref mut self) {\n        self.a = 42;\n        self.b = 77;\n    }\n\n    fn g(ref mut self, inc: u64) {\n        self.a = self.a + inc;\n        self.b = self.b + inc;\n    }\n\n    fn h(ref mut self) {\n        self = A {\n            a: 100,\n            b: 200,\n        }\n    }\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 381,
                                        end: 385,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 386,
                                        end: 387,
                                        as_str(): "E",
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 394,
                                                                end: 395,
                                                                as_str(): "X",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 395,
                                                                end: 396,
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
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 397,
                                                                            end: 400,
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
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 400,
                                                        end: 401,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                            (
                                                Annotated {
                                                    attribute_list: [],
                                                    value: TypeField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 406,
                                                                end: 407,
                                                                as_str(): "Y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 407,
                                                                end: 408,
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
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 409,
                                                                            end: 412,
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
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 412,
                                                        end: 413,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 388,
                                        end: 415,
                                        as_str(): "{\n    X: u64,\n    Y: u64,\n}",
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
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 417,
                                        end: 421,
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
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 422,
                                                    end: 423,
                                                    as_str(): "E",
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 430,
                                                            end: 432,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 433,
                                                            end: 434,
                                                            as_str(): "j",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 443,
                                                                    end: 447,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: Some(
                                                                RefToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 435,
                                                                        end: 438,
                                                                        as_str(): "ref",
                                                                    },
                                                                },
                                                            ),
                                                            mutable_self: Some(
                                                                MutToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 439,
                                                                        end: 442,
                                                                        as_str(): "mut",
                                                                    },
                                                                },
                                                            ),
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 447,
                                                                            end: 448,
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
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 449,
                                                                                            end: 452,
                                                                                            as_str(): "inc",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 452,
                                                                                        end: 453,
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
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 454,
                                                                                                    end: 457,
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 434,
                                                            end: 458,
                                                            as_str(): "(ref mut self, inc: u64)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: Some(
                                                            Reassignment {
                                                                assignable: Var(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 469,
                                                                            end: 473,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                reassignment_op: ReassignmentOp {
                                                                    variant: Equals,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 474,
                                                                        end: 475,
                                                                        as_str(): "=",
                                                                    },
                                                                },
                                                                expr: Match {
                                                                    match_token: MatchToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 476,
                                                                            end: 481,
                                                                            as_str(): "match",
                                                                        },
                                                                    },
                                                                    value: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 482,
                                                                                        end: 486,
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
                                                                    branches: Braces {
                                                                        inner: [
                                                                            MatchBranch {
                                                                                pattern: Constructor {
                                                                                    path: PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 501,
                                                                                                    end: 502,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 502,
                                                                                                        end: 504,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 504,
                                                                                                            end: 505,
                                                                                                            as_str(): "X",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ],
                                                                                        incomplete_suffix: false,
                                                                                    },
                                                                                    args: Parens {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [],
                                                                                            final_value_opt: Some(
                                                                                                Var {
                                                                                                    reference: None,
                                                                                                    mutable: None,
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 506,
                                                                                                            end: 509,
                                                                                                            as_str(): "val",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 505,
                                                                                            end: 510,
                                                                                            as_str(): "(val)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 511,
                                                                                        end: 513,
                                                                                        as_str(): "=>",
                                                                                    },
                                                                                },
                                                                                kind: Expr {
                                                                                    expr: FuncApp {
                                                                                        func: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 514,
                                                                                                            end: 515,
                                                                                                            as_str(): "E",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                                suffix: [
                                                                                                    (
                                                                                                        DoubleColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 515,
                                                                                                                end: 517,
                                                                                                                as_str(): "::",
                                                                                                            },
                                                                                                        },
                                                                                                        PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 517,
                                                                                                                    end: 518,
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
                                                                                                    Add {
                                                                                                        lhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 519,
                                                                                                                            end: 522,
                                                                                                                            as_str(): "val",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                                incomplete_suffix: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        add_token: AddToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 523,
                                                                                                                end: 524,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 525,
                                                                                                                            end: 528,
                                                                                                                            as_str(): "inc",
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
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 518,
                                                                                                end: 529,
                                                                                                as_str(): "(val + inc)",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    comma_token: CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 529,
                                                                                            end: 530,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                pattern: Constructor {
                                                                                    path: PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 543,
                                                                                                    end: 544,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            generics_opt: None,
                                                                                        },
                                                                                        suffix: [
                                                                                            (
                                                                                                DoubleColonToken {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                        ),
                                                                                                        start: 544,
                                                                                                        end: 546,
                                                                                                        as_str(): "::",
                                                                                                    },
                                                                                                },
                                                                                                PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 546,
                                                                                                            end: 547,
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
                                                                                    args: Parens {
                                                                                        inner: Punctuated {
                                                                                            value_separator_pairs: [],
                                                                                            final_value_opt: Some(
                                                                                                Var {
                                                                                                    reference: None,
                                                                                                    mutable: None,
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 548,
                                                                                                            end: 551,
                                                                                                            as_str(): "val",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 547,
                                                                                            end: 552,
                                                                                            as_str(): "(val)",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 553,
                                                                                        end: 555,
                                                                                        as_str(): "=>",
                                                                                    },
                                                                                },
                                                                                kind: Expr {
                                                                                    expr: FuncApp {
                                                                                        func: Path(
                                                                                            PathExpr {
                                                                                                root_opt: None,
                                                                                                prefix: PathExprSegment {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                            ),
                                                                                                            start: 556,
                                                                                                            end: 557,
                                                                                                            as_str(): "E",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    generics_opt: None,
                                                                                                },
                                                                                                suffix: [
                                                                                                    (
                                                                                                        DoubleColonToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 557,
                                                                                                                end: 559,
                                                                                                                as_str(): "::",
                                                                                                            },
                                                                                                        },
                                                                                                        PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 559,
                                                                                                                    end: 560,
                                                                                                                    as_str(): "X",
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
                                                                                                    Add {
                                                                                                        lhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 561,
                                                                                                                            end: 564,
                                                                                                                            as_str(): "val",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    generics_opt: None,
                                                                                                                },
                                                                                                                suffix: [],
                                                                                                                incomplete_suffix: false,
                                                                                                            },
                                                                                                        ),
                                                                                                        add_token: AddToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                ),
                                                                                                                start: 565,
                                                                                                                end: 566,
                                                                                                                as_str(): "+",
                                                                                                            },
                                                                                                        },
                                                                                                        rhs: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 567,
                                                                                                                            end: 570,
                                                                                                                            as_str(): "inc",
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
                                                                                                ),
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 560,
                                                                                                end: 571,
                                                                                                as_str(): "(val + inc)",
                                                                                            },
                                                                                        },
                                                                                    },
                                                                                    comma_token: CommaToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 571,
                                                                                            end: 572,
                                                                                            as_str(): ",",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 487,
                                                                            end: 582,
                                                                            as_str(): "{\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe04ae0ff10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                        ),
                                                        start: 459,
                                                        end: 588,
                                                        as_str(): "{\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 424,
                                        end: 590,
                                        as_str(): "{\n    fn j(ref mut self, inc: u64) {\n        self = match self {\n            E::X(val) => E::Y(val + inc),\n            E::Y(val) => E::X(val + inc),\n        }\n    }\n}",
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
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 592,
                                            end: 594,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 595,
                                            end: 599,
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
                                            src (ptr): 0x00007fe04ae0ff10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                            ),
                                            start: 599,
                                            end: 601,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe04ae0ff10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                    ),
                                                    start: 602,
                                                    end: 604,
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
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 605,
                                                                end: 609,
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
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 616,
                                                            end: 619,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 620,
                                                                    end: 623,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 624,
                                                                end: 625,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 626,
                                                            end: 627,
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
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 628,
                                                                        end: 629,
                                                                        as_str(): "A",
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
                                                                value_separator_pairs: [
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 640,
                                                                                    end: 641,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 641,
                                                                                            end: 642,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 643,
                                                                                                    end: 644,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 644,
                                                                                end: 645,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 654,
                                                                                    end: 655,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 655,
                                                                                            end: 656,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 657,
                                                                                                    end: 658,
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
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 658,
                                                                                end: 659,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 630,
                                                                end: 665,
                                                                as_str(): "{\n        a: 0,\n        b: 0,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 665,
                                                            end: 666,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 672,
                                                                        end: 673,
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
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 673,
                                                            end: 674,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 674,
                                                                end: 675,
                                                                as_str(): "f",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 675,
                                                            end: 677,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 677,
                                                            end: 678,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 683,
                                                                        end: 689,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 690,
                                                                                            end: 691,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 691,
                                                                                end: 692,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 692,
                                                                                end: 693,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 694,
                                                                            end: 696,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 697,
                                                                                    end: 699,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 689,
                                                            end: 700,
                                                            as_str(): "(a.a == 42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 700,
                                                            end: 701,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 706,
                                                                        end: 712,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 713,
                                                                                            end: 714,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 714,
                                                                                end: 715,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 715,
                                                                                end: 716,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 717,
                                                                            end: 719,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 720,
                                                                                    end: 722,
                                                                                    as_str(): "77",
                                                                                },
                                                                                parsed: 77,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 712,
                                                            end: 723,
                                                            as_str(): "(a.b == 77)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 723,
                                                            end: 724,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 730,
                                                                        end: 731,
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
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 731,
                                                            end: 732,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 732,
                                                                end: 733,
                                                                as_str(): "g",
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
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 734,
                                                                                end: 735,
                                                                                as_str(): "1",
                                                                            },
                                                                            parsed: 1,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 733,
                                                            end: 736,
                                                            as_str(): "(1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 736,
                                                            end: 737,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 742,
                                                                        end: 748,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 749,
                                                                                            end: 750,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 750,
                                                                                end: 751,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 751,
                                                                                end: 752,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 753,
                                                                            end: 755,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 756,
                                                                                    end: 758,
                                                                                    as_str(): "43",
                                                                                },
                                                                                parsed: 43,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 748,
                                                            end: 759,
                                                            as_str(): "(a.a == 43)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 759,
                                                            end: 760,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 765,
                                                                        end: 771,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 772,
                                                                                            end: 773,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 773,
                                                                                end: 774,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 774,
                                                                                end: 775,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 776,
                                                                            end: 778,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 779,
                                                                                    end: 781,
                                                                                    as_str(): "78",
                                                                                },
                                                                                parsed: 78,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 771,
                                                            end: 782,
                                                            as_str(): "(a.b == 78)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 782,
                                                            end: 783,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 789,
                                                                        end: 790,
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
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 790,
                                                            end: 791,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 791,
                                                                end: 792,
                                                                as_str(): "h",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 792,
                                                            end: 794,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 794,
                                                            end: 795,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 800,
                                                                        end: 806,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 807,
                                                                                            end: 808,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 808,
                                                                                end: 809,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 809,
                                                                                end: 810,
                                                                                as_str(): "a",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 811,
                                                                            end: 813,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 814,
                                                                                    end: 817,
                                                                                    as_str(): "100",
                                                                                },
                                                                                parsed: 100,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 806,
                                                            end: 818,
                                                            as_str(): "(a.a == 100)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 818,
                                                            end: 819,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 824,
                                                                        end: 830,
                                                                        as_str(): "assert",
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
                                                                Equal {
                                                                    lhs: FieldProjection {
                                                                        target: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 831,
                                                                                            end: 832,
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
                                                                        dot_token: DotToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 832,
                                                                                end: 833,
                                                                                as_str(): ".",
                                                                            },
                                                                        },
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 833,
                                                                                end: 834,
                                                                                as_str(): "b",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 835,
                                                                            end: 837,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 838,
                                                                                    end: 841,
                                                                                    as_str(): "200",
                                                                                },
                                                                                parsed: 200,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 830,
                                                            end: 842,
                                                            as_str(): "(a.b == 200)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 842,
                                                            end: 843,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 849,
                                                            end: 852,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                    ),
                                                                    start: 853,
                                                                    end: 856,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 857,
                                                                end: 858,
                                                                as_str(): "e",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 859,
                                                            end: 860,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: FuncApp {
                                                        func: Path(
                                                            PathExpr {
                                                                root_opt: None,
                                                                prefix: PathExprSegment {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 861,
                                                                            end: 862,
                                                                            as_str(): "E",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 862,
                                                                                end: 864,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 864,
                                                                                    end: 865,
                                                                                    as_str(): "X",
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
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 866,
                                                                                    end: 868,
                                                                                    as_str(): "42",
                                                                                },
                                                                                parsed: 42,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 865,
                                                                end: 869,
                                                                as_str(): "(42)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 869,
                                                            end: 870,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 875,
                                                            end: 880,
                                                            as_str(): "match",
                                                        },
                                                    },
                                                    value: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 881,
                                                                        end: 882,
                                                                        as_str(): "e",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    branches: Braces {
                                                        inner: [
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 893,
                                                                                    end: 894,
                                                                                    as_str(): "E",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 894,
                                                                                        end: 896,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 896,
                                                                                            end: 897,
                                                                                            as_str(): "X",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 898,
                                                                                                end: 900,
                                                                                                as_str(): "42",
                                                                                            },
                                                                                            parsed: 42,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 897,
                                                                            end: 901,
                                                                            as_str(): "(42)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 902,
                                                                        end: 904,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 905,
                                                                            end: 907,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 907,
                                                                                end: 908,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Wildcard {
                                                                    underscore_token: UnderscoreToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 917,
                                                                            end: 918,
                                                                            as_str(): "_",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 919,
                                                                        end: 921,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 922,
                                                                                            end: 928,
                                                                                            as_str(): "revert",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 929,
                                                                                                    end: 930,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 928,
                                                                                end: 931,
                                                                                as_str(): "(0)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 931,
                                                                            end: 932,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 883,
                                                            end: 938,
                                                            as_str(): "{\n        E::X(42) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 938,
                                                            end: 939,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 949,
                                                                        end: 950,
                                                                        as_str(): "e",
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 950,
                                                            end: 951,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 951,
                                                                end: 952,
                                                                as_str(): "j",
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
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 953,
                                                                                end: 954,
                                                                                as_str(): "4",
                                                                            },
                                                                            parsed: 4,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 952,
                                                            end: 955,
                                                            as_str(): "(4)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 955,
                                                            end: 956,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 961,
                                                            end: 966,
                                                            as_str(): "match",
                                                        },
                                                    },
                                                    value: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 967,
                                                                        end: 968,
                                                                        as_str(): "e",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    branches: Braces {
                                                        inner: [
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 979,
                                                                                    end: 980,
                                                                                    as_str(): "E",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 980,
                                                                                        end: 982,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 982,
                                                                                            end: 983,
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
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 984,
                                                                                                end: 986,
                                                                                                as_str(): "46",
                                                                                            },
                                                                                            parsed: 46,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 983,
                                                                            end: 987,
                                                                            as_str(): "(46)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 988,
                                                                        end: 990,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 991,
                                                                            end: 993,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 993,
                                                                                end: 994,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Wildcard {
                                                                    underscore_token: UnderscoreToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1003,
                                                                            end: 1004,
                                                                            as_str(): "_",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 1005,
                                                                        end: 1007,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 1008,
                                                                                            end: 1014,
                                                                                            as_str(): "revert",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1015,
                                                                                                    end: 1016,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1014,
                                                                                end: 1017,
                                                                                as_str(): "(0)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1017,
                                                                            end: 1018,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 969,
                                                            end: 1024,
                                                            as_str(): "{\n        E::Y(46) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1024,
                                                            end: 1025,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 1031,
                                                                        end: 1032,
                                                                        as_str(): "e",
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
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1032,
                                                            end: 1033,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                ),
                                                                start: 1033,
                                                                end: 1034,
                                                                as_str(): "j",
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
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1035,
                                                                                end: 1036,
                                                                                as_str(): "5",
                                                                            },
                                                                            parsed: 5,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1034,
                                                            end: 1037,
                                                            as_str(): "(5)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1037,
                                                            end: 1038,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: Match {
                                                    match_token: MatchToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1043,
                                                            end: 1048,
                                                            as_str(): "match",
                                                        },
                                                    },
                                                    value: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 1049,
                                                                        end: 1050,
                                                                        as_str(): "e",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    branches: Braces {
                                                        inner: [
                                                            MatchBranch {
                                                                pattern: Constructor {
                                                                    path: PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                    ),
                                                                                    start: 1061,
                                                                                    end: 1062,
                                                                                    as_str(): "E",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: None,
                                                                        },
                                                                        suffix: [
                                                                            (
                                                                                DoubleColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                        ),
                                                                                        start: 1062,
                                                                                        end: 1064,
                                                                                        as_str(): "::",
                                                                                    },
                                                                                },
                                                                                PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 1064,
                                                                                            end: 1065,
                                                                                            as_str(): "X",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                            ),
                                                                        ],
                                                                        incomplete_suffix: false,
                                                                    },
                                                                    args: Parens {
                                                                        inner: Punctuated {
                                                                            value_separator_pairs: [],
                                                                            final_value_opt: Some(
                                                                                Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                ),
                                                                                                start: 1066,
                                                                                                end: 1068,
                                                                                                as_str(): "51",
                                                                                            },
                                                                                            parsed: 51,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1065,
                                                                            end: 1069,
                                                                            as_str(): "(51)",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 1070,
                                                                        end: 1072,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Block {
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1073,
                                                                            end: 1075,
                                                                            as_str(): "{}",
                                                                        },
                                                                    },
                                                                    comma_token_opt: Some(
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1075,
                                                                                end: 1076,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                            },
                                                            MatchBranch {
                                                                pattern: Wildcard {
                                                                    underscore_token: UnderscoreToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1085,
                                                                            end: 1086,
                                                                            as_str(): "_",
                                                                        },
                                                                    },
                                                                },
                                                                fat_right_arrow_token: FatRightArrowToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe04ae0ff10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                        ),
                                                                        start: 1087,
                                                                        end: 1089,
                                                                        as_str(): "=>",
                                                                    },
                                                                },
                                                                kind: Expr {
                                                                    expr: FuncApp {
                                                                        func: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                            ),
                                                                                            start: 1090,
                                                                                            end: 1096,
                                                                                            as_str(): "revert",
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
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe04ae0ff10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1097,
                                                                                                    end: 1098,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                                parsed: 0,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe04ae0ff10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                                ),
                                                                                start: 1096,
                                                                                end: 1099,
                                                                                as_str(): "(0)",
                                                                            },
                                                                        },
                                                                    },
                                                                    comma_token: CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe04ae0ff10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                                            ),
                                                                            start: 1099,
                                                                            end: 1100,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1051,
                                                            end: 1106,
                                                            as_str(): "{\n        E::X(51) => {},\n        _ => revert(0),\n    }",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1106,
                                                            end: 1107,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Bool(
                                                    LitBool {
                                                        span: Span {
                                                            src (ptr): 0x00007fe04ae0ff10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                                            ),
                                                            start: 1116,
                                                            end: 1120,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe04ae0ff10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRaNPqV4/self_impl_reassignment/src/main.sw",
                                        ),
                                        start: 610,
                                        end: 1122,
                                        as_str(): "{\n    let mut a = A {\n        a: 0,\n        b: 0,\n    };\n\n    a.f();\n    assert(a.a == 42);\n    assert(a.b == 77);\n\n    a.g(1);\n    assert(a.a == 43);\n    assert(a.b == 78);\n\n    a.h();\n    assert(a.a == 100);\n    assert(a.b == 200);\n\n    let mut e = E::X(42);\n    match e {\n        E::X(42) => {},\n        _ => revert(0),\n    };\n    \n    e.j(4);\n    match e {\n        E::Y(46) => {},\n        _ => revert(0),\n    };\n\n    e.j(5);\n    match e {\n        E::X(51) => {},\n        _ => revert(0),\n    };\n   \n    true\n}",
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
