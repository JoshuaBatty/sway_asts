Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0b4550000,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0b4550000,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 9,
                                            end: 11,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 12,
                                            end: 15,
                                            as_str(): "log",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: Some(
                                        GenericParams {
                                            parameters: AngleBrackets {
                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 15,
                                                        end: 16,
                                                        as_str(): "<",
                                                    },
                                                },
                                                inner: Punctuated {
                                                    value_separator_pairs: [],
                                                    final_value_opt: Some(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 16,
                                                                end: 17,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                },
                                                close_angle_bracket_token: CloseAngleBracketToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 17,
                                                        end: 18,
                                                        as_str(): ">",
                                                    },
                                                },
                                            },
                                        },
                                    ),
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
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 19,
                                                                    end: 24,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 24,
                                                                end: 25,
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
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 26,
                                                                            end: 27,
                                                                            as_str(): "T",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 18,
                                            end: 28,
                                            as_str(): "(value: T)",
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 35,
                                                                        end: 40,
                                                                        as_str(): "__log",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: Some(
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 40,
                                                                                end: 42,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        GenericArgs {
                                                                            parameters: AngleBrackets {
                                                                                open_angle_bracket_token: OpenAngleBracketToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 42,
                                                                                        end: 43,
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
                                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                            ),
                                                                                                            start: 43,
                                                                                                            end: 44,
                                                                                                            as_str(): "T",
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
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 44,
                                                                                        end: 45,
                                                                                        as_str(): ">",
                                                                                    },
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
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
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 46,
                                                                                    end: 51,
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
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 45,
                                                            end: 52,
                                                            as_str(): "(value)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 52,
                                                            end: 53,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 29,
                                        end: 55,
                                        as_str(): "{\n    __log::<T>(value);\n}",
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
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 57,
                                        end: 63,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 74,
                                        as_str(): "TestStruct",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 74,
                                                    end: 75,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 76,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 76,
                                                    end: 77,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 84,
                                                                end: 91,
                                                                as_str(): "field_1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 91,
                                                                end: 92,
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
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 93,
                                                                            end: 97,
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
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 97,
                                                        end: 98,
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 103,
                                                                end: 110,
                                                                as_str(): "field_2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 110,
                                                                end: 111,
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
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 112,
                                                                            end: 113,
                                                                            as_str(): "T",
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
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 113,
                                                        end: 114,
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 119,
                                                                end: 126,
                                                                as_str(): "field_3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 126,
                                                                end: 127,
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
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 128,
                                                                            end: 131,
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
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 131,
                                                        end: 132,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 78,
                                        end: 134,
                                        as_str(): "{\n    field_1: bool,\n    field_2: T,\n    field_3: u64,\n}",
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
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 136,
                                        end: 140,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 141,
                                        end: 149,
                                        as_str(): "TestEnum",
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 156,
                                                                end: 166,
                                                                as_str(): "VariantOne",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 166,
                                                                end: 167,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 168,
                                                                    end: 170,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 170,
                                                        end: 171,
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 176,
                                                                end: 186,
                                                                as_str(): "VariantTwo",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 186,
                                                                end: 187,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 188,
                                                                    end: 190,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 190,
                                                        end: 191,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 193,
                                        as_str(): "{\n    VariantOne: (),\n    VariantTwo: (),\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: Some(
                                    PubToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 195,
                                            end: 198,
                                            as_str(): "pub",
                                        },
                                    },
                                ),
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 199,
                                        end: 203,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 204,
                                        end: 210,
                                        as_str(): "Option",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 210,
                                                    end: 211,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 211,
                                                            end: 212,
                                                            as_str(): "T",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 212,
                                                    end: 213,
                                                    as_str(): ">",
                                                },
                                            },
                                        },
                                    },
                                ),
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 220,
                                                                end: 224,
                                                                as_str(): "None",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 224,
                                                                end: 225,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Tuple(
                                                            Parens {
                                                                inner: Nil,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 226,
                                                                    end: 228,
                                                                    as_str(): "()",
                                                                },
                                                            },
                                                        ),
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 228,
                                                        end: 229,
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 234,
                                                                end: 238,
                                                                as_str(): "Some",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 238,
                                                                end: 239,
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
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 240,
                                                                            end: 241,
                                                                            as_str(): "T",
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
                                                        src (ptr): 0x00007fe0b4550000,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                        ),
                                                        start: 241,
                                                        end: 242,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 214,
                                        end: 244,
                                        as_str(): "{\n    None: (),\n    Some: T,\n}",
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 246,
                                            end: 248,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 249,
                                            end: 253,
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
                                            src (ptr): 0x00007fe0b4550000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                            ),
                                            start: 253,
                                            end: 255,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0b4550000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                    ),
                                                    start: 256,
                                                    end: 258,
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
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 259,
                                                                end: 263,
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 273,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 274,
                                                                end: 275,
                                                                as_str(): "k",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 275,
                                                                    end: 276,
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
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 277,
                                                                                end: 281,
                                                                                as_str(): "b256",
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 283,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 284,
                                                                    end: 350,
                                                                    as_str(): "0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a",
                                                                },
                                                                parsed: 108340740691025251715289040289762951404984910512315932338997285918435378372170,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 350,
                                                            end: 351,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 356,
                                                            end: 359,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 360,
                                                                end: 361,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 361,
                                                                    end: 362,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Str {
                                                                str_token: StrToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 363,
                                                                        end: 366,
                                                                        as_str(): "str",
                                                                    },
                                                                },
                                                                length: SquareBrackets {
                                                                    inner: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 367,
                                                                                    end: 368,
                                                                                    as_str(): "4",
                                                                                },
                                                                                parsed: 4,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 366,
                                                                        end: 369,
                                                                        as_str(): "[4]",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 370,
                                                            end: 371,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        String(
                                                            LitString {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 372,
                                                                    end: 378,
                                                                    as_str(): "\"Fuel\"",
                                                                },
                                                                parsed: "Fuel",
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 378,
                                                            end: 379,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 384,
                                                            end: 387,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 388,
                                                                end: 389,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0b4550000,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                    ),
                                                                    start: 389,
                                                                    end: 390,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Array(
                                                                SquareBrackets {
                                                                    inner: TyArrayDescriptor {
                                                                        ty: Path(
                                                                            PathType {
                                                                                root_opt: None,
                                                                                prefix: PathTypeSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 392,
                                                                                            end: 394,
                                                                                            as_str(): "u8",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                            },
                                                                        ),
                                                                        semicolon_token: SemicolonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 394,
                                                                                end: 395,
                                                                                as_str(): ";",
                                                                            },
                                                                        },
                                                                        length: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 396,
                                                                                        end: 397,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 391,
                                                                        end: 398,
                                                                        as_str(): "[u8; 3]",
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 399,
                                                            end: 400,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Array(
                                                        SquareBrackets {
                                                            inner: Sequence(
                                                                Punctuated {
                                                                    value_separator_pairs: [
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 402,
                                                                                            end: 403,
                                                                                            as_str(): "1",
                                                                                        },
                                                                                        parsed: 1,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U8,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                    ),
                                                                                                    start: 403,
                                                                                                    end: 405,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 405,
                                                                                    end: 406,
                                                                                    as_str(): ",",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            Literal(
                                                                                Int(
                                                                                    LitInt {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 407,
                                                                                            end: 408,
                                                                                            as_str(): "2",
                                                                                        },
                                                                                        parsed: 2,
                                                                                        ty_opt: Some(
                                                                                            (
                                                                                                U8,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                    ),
                                                                                                    start: 408,
                                                                                                    end: 410,
                                                                                                    as_str(): "u8",
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            CommaToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 410,
                                                                                    end: 411,
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
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 412,
                                                                                        end: 413,
                                                                                        as_str(): "3",
                                                                                    },
                                                                                    parsed: 3,
                                                                                    ty_opt: Some(
                                                                                        (
                                                                                            U8,
                                                                                            Span {
                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                ),
                                                                                                start: 413,
                                                                                                end: 415,
                                                                                                as_str(): "u8",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 401,
                                                                end: 416,
                                                                as_str(): "[1u8, 2u8, 3u8]",
                                                            },
                                                        },
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 416,
                                                            end: 417,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 422,
                                                            end: 425,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 426,
                                                                end: 437,
                                                                as_str(): "test_struct",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 438,
                                                            end: 439,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 440,
                                                                        end: 450,
                                                                        as_str(): "TestStruct",
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
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 461,
                                                                                    end: 468,
                                                                                    as_str(): "field_1",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 468,
                                                                                            end: 469,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Bool(
                                                                                            LitBool {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                    ),
                                                                                                    start: 470,
                                                                                                    end: 474,
                                                                                                    as_str(): "true",
                                                                                                },
                                                                                                kind: True,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 474,
                                                                                end: 475,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 484,
                                                                                    end: 491,
                                                                                    as_str(): "field_2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 491,
                                                                                            end: 492,
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
                                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                        ),
                                                                                                        start: 493,
                                                                                                        end: 494,
                                                                                                        as_str(): "k",
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
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 494,
                                                                                end: 495,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                    (
                                                                        ExprStructField {
                                                                            field_name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 504,
                                                                                    end: 511,
                                                                                    as_str(): "field_3",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            expr_opt: Some(
                                                                                (
                                                                                    ColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 511,
                                                                                            end: 512,
                                                                                            as_str(): ":",
                                                                                        },
                                                                                    },
                                                                                    Literal(
                                                                                        Int(
                                                                                            LitInt {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                    ),
                                                                                                    start: 513,
                                                                                                    end: 515,
                                                                                                    as_str(): "11",
                                                                                                },
                                                                                                parsed: 11,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            ),
                                                                        },
                                                                        CommaToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 515,
                                                                                end: 516,
                                                                                as_str(): ",",
                                                                            },
                                                                        },
                                                                    ),
                                                                ],
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 451,
                                                                end: 522,
                                                                as_str(): "{\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    }",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 522,
                                                            end: 523,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 529,
                                                            end: 532,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0b4550000,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                ),
                                                                start: 533,
                                                                end: 542,
                                                                as_str(): "test_enum",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 543,
                                                            end: 544,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 545,
                                                                        end: 553,
                                                                        as_str(): "TestEnum",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [
                                                                (
                                                                    DoubleColonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 553,
                                                                            end: 555,
                                                                            as_str(): "::",
                                                                        },
                                                                    },
                                                                    PathExprSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 555,
                                                                                end: 565,
                                                                                as_str(): "VariantTwo",
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
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 565,
                                                            end: 566,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 571,
                                                                        end: 574,
                                                                        as_str(): "log",
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
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 575,
                                                                                    end: 576,
                                                                                    as_str(): "k",
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 574,
                                                            end: 577,
                                                            as_str(): "(k)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 577,
                                                            end: 578,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 583,
                                                                        end: 586,
                                                                        as_str(): "log",
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
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 587,
                                                                                end: 589,
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 586,
                                                            end: 590,
                                                            as_str(): "(42)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 590,
                                                            end: 591,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 596,
                                                                        end: 599,
                                                                        as_str(): "log",
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
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 600,
                                                                                end: 602,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U32,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 602,
                                                                                        end: 605,
                                                                                        as_str(): "u32",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 599,
                                                            end: 606,
                                                            as_str(): "(42u32)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 606,
                                                            end: 607,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 612,
                                                                        end: 615,
                                                                        as_str(): "log",
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
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 616,
                                                                                end: 618,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U16,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 618,
                                                                                        end: 621,
                                                                                        as_str(): "u16",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 615,
                                                            end: 622,
                                                            as_str(): "(42u16)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 622,
                                                            end: 623,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 628,
                                                                        end: 631,
                                                                        as_str(): "log",
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
                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                ),
                                                                                start: 632,
                                                                                end: 634,
                                                                                as_str(): "42",
                                                                            },
                                                                            parsed: 42,
                                                                            ty_opt: Some(
                                                                                (
                                                                                    U8,
                                                                                    Span {
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 634,
                                                                                        end: 636,
                                                                                        as_str(): "u8",
                                                                                    },
                                                                                ),
                                                                            ),
                                                                        },
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 631,
                                                            end: 637,
                                                            as_str(): "(42u8)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 637,
                                                            end: 638,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 643,
                                                                        end: 648,
                                                                        as_str(): "__log",
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
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 649,
                                                                                    end: 650,
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 648,
                                                            end: 651,
                                                            as_str(): "(a)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 651,
                                                            end: 652,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 657,
                                                                        end: 662,
                                                                        as_str(): "__log",
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
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 663,
                                                                                    end: 664,
                                                                                    as_str(): "b",
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 662,
                                                            end: 665,
                                                            as_str(): "(b)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 665,
                                                            end: 666,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 671,
                                                                        end: 676,
                                                                        as_str(): "__log",
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
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 677,
                                                                                    end: 688,
                                                                                    as_str(): "test_struct",
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 676,
                                                            end: 689,
                                                            as_str(): "(test_struct)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 689,
                                                            end: 690,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 695,
                                                                        end: 700,
                                                                        as_str(): "__log",
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
                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                    ),
                                                                                    start: 701,
                                                                                    end: 710,
                                                                                    as_str(): "test_enum",
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 700,
                                                            end: 711,
                                                            as_str(): "(test_enum)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 711,
                                                            end: 712,
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
                                                                        src (ptr): 0x00007fe0b4550000,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                        ),
                                                                        start: 717,
                                                                        end: 722,
                                                                        as_str(): "__log",
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
                                                                FuncApp {
                                                                    func: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                        ),
                                                                                        start: 723,
                                                                                        end: 729,
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
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 729,
                                                                                            end: 731,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                ),
                                                                                                start: 731,
                                                                                                end: 735,
                                                                                                as_str(): "Some",
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
                                                                                Struct {
                                                                                    path: PathExpr {
                                                                                        root_opt: None,
                                                                                        prefix: PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0b4550000,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                    ),
                                                                                                    start: 736,
                                                                                                    end: 746,
                                                                                                    as_str(): "TestStruct",
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
                                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                ),
                                                                                                                start: 757,
                                                                                                                end: 764,
                                                                                                                as_str(): "field_1",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 764,
                                                                                                                        end: 765,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Bool(
                                                                                                                        LitBool {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 766,
                                                                                                                                end: 770,
                                                                                                                                as_str(): "true",
                                                                                                                            },
                                                                                                                            kind: True,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                            ),
                                                                                                            start: 770,
                                                                                                            end: 771,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                ),
                                                                                                                start: 780,
                                                                                                                end: 787,
                                                                                                                as_str(): "field_2",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 787,
                                                                                                                        end: 788,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 789,
                                                                                                                                end: 791,
                                                                                                                                as_str(): "42",
                                                                                                                            },
                                                                                                                            parsed: 42,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                            ),
                                                                                                            start: 791,
                                                                                                            end: 792,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    ExprStructField {
                                                                                                        field_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                ),
                                                                                                                start: 801,
                                                                                                                end: 808,
                                                                                                                as_str(): "field_3",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        expr_opt: Some(
                                                                                                            (
                                                                                                                ColonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0b4550000,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 808,
                                                                                                                        end: 809,
                                                                                                                        as_str(): ":",
                                                                                                                    },
                                                                                                                },
                                                                                                                Literal(
                                                                                                                    Int(
                                                                                                                        LitInt {
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0b4550000,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 810,
                                                                                                                                end: 812,
                                                                                                                                as_str(): "42",
                                                                                                                            },
                                                                                                                            parsed: 42,
                                                                                                                            ty_opt: None,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                ),
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    CommaToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                                            ),
                                                                                                            start: 812,
                                                                                                            end: 813,
                                                                                                            as_str(): ",",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            final_value_opt: None,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0b4550000,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                                            ),
                                                                                            start: 747,
                                                                                            end: 819,
                                                                                            as_str(): "{\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0b4550000,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                                            ),
                                                                            start: 735,
                                                                            end: 820,
                                                                            as_str(): "(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    })",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 722,
                                                            end: 821,
                                                            as_str(): "(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }))",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 821,
                                                            end: 822,
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
                                                            src (ptr): 0x00007fe0b4550000,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                                            ),
                                                            start: 828,
                                                            end: 832,
                                                            as_str(): "true",
                                                        },
                                                        kind: True,
                                                    },
                                                ),
                                            ),
                                        ),
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0b4550000,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR3ixxft/logging/src/main.sw",
                                        ),
                                        start: 264,
                                        end: 834,
                                        as_str(): "{\n    let k: b256 = 0xef86afa9696cf0dc6385e2c407a6e159a1103cefb7e2ae0636fb33d3cb2a9e4a;\n    let a: str[4] = \"Fuel\";\n    let b: [u8; 3] = [1u8, 2u8, 3u8];\n    let test_struct = TestStruct {\n        field_1: true,\n        field_2: k,\n        field_3: 11,\n    };\n\n    let test_enum = TestEnum::VariantTwo;\n    log(k);\n    log(42);\n    log(42u32);\n    log(42u16);\n    log(42u8);\n    __log(a);\n    __log(b);\n    __log(test_struct);\n    __log(test_enum);\n    __log(Option::Some(TestStruct {\n        field_1: true,\n        field_2: 42,\n        field_3: 42,\n    }));\n\n    true\n}",
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
