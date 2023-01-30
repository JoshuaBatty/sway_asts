Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe066b453c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe066b453c0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
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
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 15,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 16,
                                        end: 17,
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
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                ),
                                                                start: 24,
                                                                end: 26,
                                                                as_str(): "f1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                ),
                                                                start: 26,
                                                                end: 27,
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
                                                                            src (ptr): 0x00007fe066b453c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                            ),
                                                                            start: 28,
                                                                            end: 35,
                                                                            as_str(): "Address",
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
                                                        src (ptr): 0x00007fe066b453c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                        ),
                                                        start: 35,
                                                        end: 36,
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
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                ),
                                                                start: 41,
                                                                end: 43,
                                                                as_str(): "f2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                ),
                                                                start: 43,
                                                                end: 44,
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
                                                                            src (ptr): 0x00007fe066b453c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                            ),
                                                                            start: 45,
                                                                            end: 55,
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
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe066b453c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                        ),
                                                        start: 55,
                                                        end: 56,
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
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                ),
                                                                start: 61,
                                                                end: 63,
                                                                as_str(): "f3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                ),
                                                                start: 63,
                                                                end: 64,
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
                                                                            src (ptr): 0x00007fe066b453c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                            ),
                                                                            start: 65,
                                                                            end: 73,
                                                                            as_str(): "Identity",
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
                                                        src (ptr): 0x00007fe066b453c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                        ),
                                                        start: 73,
                                                        end: 74,
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
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                ),
                                                                start: 79,
                                                                end: 81,
                                                                as_str(): "f4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe066b453c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
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
                                                                            src (ptr): 0x00007fe066b453c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                            ),
                                                                            start: 83,
                                                                            end: 86,
                                                                            as_str(): "Vec",
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
                                                                                            src (ptr): 0x00007fe066b453c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                                            ),
                                                                                            start: 86,
                                                                                            end: 87,
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
                                                                                                                src (ptr): 0x00007fe066b453c0,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                                                                ),
                                                                                                                start: 87,
                                                                                                                end: 89,
                                                                                                                as_str(): "u8",
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
                                                                                            src (ptr): 0x00007fe066b453c0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                                            ),
                                                                                            start: 89,
                                                                                            end: 90,
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
                                                        src (ptr): 0x00007fe066b453c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                        ),
                                                        start: 90,
                                                        end: 91,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 18,
                                        end: 93,
                                        as_str(): "{\n    f1: Address,\n    f2: ContractId,\n    f3: Identity,\n    f4: Vec<u8>,\n}",
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
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 95,
                                            end: 97,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 98,
                                            end: 101,
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
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 101,
                                            end: 103,
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
                                                expr: FuncApp {
                                                    func: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe066b453c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                        ),
                                                                        start: 110,
                                                                        end: 116,
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
                                                                Literal(
                                                                    Bool(
                                                                        LitBool {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe066b453c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                                ),
                                                                                start: 117,
                                                                                end: 121,
                                                                                as_str(): "true",
                                                                            },
                                                                            kind: True,
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe066b453c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                            ),
                                                            start: 116,
                                                            end: 122,
                                                            as_str(): "(true)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe066b453c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                            ),
                                                            start: 122,
                                                            end: 123,
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
                                                                        src (ptr): 0x00007fe066b453c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                        ),
                                                                        start: 128,
                                                                        end: 135,
                                                                        as_str(): "require",
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
                                                            value_separator_pairs: [
                                                                (
                                                                    Literal(
                                                                        Bool(
                                                                            LitBool {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe066b453c0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                                    ),
                                                                                    start: 136,
                                                                                    end: 140,
                                                                                    as_str(): "true",
                                                                                },
                                                                                kind: True,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe066b453c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                            ),
                                                                            start: 140,
                                                                            end: 141,
                                                                            as_str(): ",",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            final_value_opt: Some(
                                                                Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe066b453c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                                ),
                                                                                start: 142,
                                                                                end: 143,
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
                                                            src (ptr): 0x00007fe066b453c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                            ),
                                                            start: 135,
                                                            end: 144,
                                                            as_str(): "(true, 0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe066b453c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                            ),
                                                            start: 144,
                                                            end: 145,
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
                                                                        src (ptr): 0x00007fe066b453c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                        ),
                                                                        start: 150,
                                                                        end: 156,
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
                                                                                src (ptr): 0x00007fe066b453c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                                                ),
                                                                                start: 157,
                                                                                end: 158,
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
                                                            src (ptr): 0x00007fe066b453c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                            ),
                                                            start: 156,
                                                            end: 159,
                                                            as_str(): "(0)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe066b453c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                                            ),
                                                            start: 159,
                                                            end: 160,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 104,
                                        end: 162,
                                        as_str(): "{\n    assert(true);\n    require(true, 0);\n    revert(0);\n}",
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
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 164,
                                            end: 166,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 167,
                                            end: 171,
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
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 171,
                                            end: 173,
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
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 174,
                                        end: 177,
                                        as_str(): "{\n}",
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
