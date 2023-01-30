Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb0ed8e1980,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb0ed8e1980,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Enum(
                            ItemEnum {
                                visibility: None,
                                enum_token: EnumToken {
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 170,
                                        end: 174,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 175,
                                        end: 181,
                                        as_str(): "Result",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: Some(
                                    GenericParams {
                                        parameters: AngleBrackets {
                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 181,
                                                    end: 182,
                                                    as_str(): "<",
                                                },
                                            },
                                            inner: Punctuated {
                                                value_separator_pairs: [
                                                    (
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 182,
                                                                end: 183,
                                                                as_str(): "T",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        CommaToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 183,
                                                                end: 184,
                                                                as_str(): ",",
                                                            },
                                                        },
                                                    ),
                                                ],
                                                final_value_opt: Some(
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 186,
                                                            as_str(): "E",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                            },
                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 186,
                                                    end: 187,
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
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 192,
                                                                end: 194,
                                                                as_str(): "Ok",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 194,
                                                                end: 195,
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
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 196,
                                                                            end: 197,
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
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 197,
                                                        end: 198,
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
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 201,
                                                                end: 204,
                                                                as_str(): "Err",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 204,
                                                                end: 205,
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
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 206,
                                                                            end: 207,
                                                                            as_str(): "E",
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
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 207,
                                                        end: 208,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 188,
                                        end: 210,
                                        as_str(): "{\n  Ok: T,\n  Err: E,\n}",
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
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 212,
                                        end: 218,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 219,
                                        end: 226,
                                        as_str(): "Product",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 227,
                                        end: 230,
                                        as_str(): "{\n}",
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
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 232,
                                        end: 238,
                                        as_str(): "struct",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 239,
                                        end: 250,
                                        as_str(): "ItemDetails",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                fields: Braces {
                                    inner: Punctuated {
                                        value_separator_pairs: [],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 251,
                                        end: 254,
                                        as_str(): "{\n}",
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
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 256,
                                        end: 260,
                                        as_str(): "enum",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 261,
                                        end: 270,
                                        as_str(): "SaleError",
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
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 277,
                                                                end: 295,
                                                                as_str(): "NotEnoughInventory",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 295,
                                                                end: 296,
                                                                as_str(): ":",
                                                            },
                                                        },
                                                        ty: Str {
                                                            str_token: StrToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 297,
                                                                    end: 300,
                                                                    as_str(): "str",
                                                                },
                                                            },
                                                            length: SquareBrackets {
                                                                inner: Literal(
                                                                    Int(
                                                                        LitInt {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 301,
                                                                                end: 302,
                                                                                as_str(): "3",
                                                                            },
                                                                            parsed: 3,
                                                                            ty_opt: None,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 300,
                                                                    end: 303,
                                                                    as_str(): "[3]",
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                                CommaToken {
                                                    span: Span {
                                                        src (ptr): 0x00007fb0ed8e1980,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                        ),
                                                        start: 303,
                                                        end: 304,
                                                        as_str(): ",",
                                                    },
                                                },
                                            ),
                                        ],
                                        final_value_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 271,
                                        end: 307,
                                        as_str(): "{\n    NotEnoughInventory: str[3], \n}",
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
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 309,
                                            end: 311,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 312,
                                            end: 316,
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
                                            src (ptr): 0x00007fb0ed8e1980,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                            ),
                                            start: 316,
                                            end: 318,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb0ed8e1980,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                    ),
                                                    start: 319,
                                                    end: 321,
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
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 322,
                                                                end: 325,
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
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 332,
                                                            end: 335,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 336,
                                                                end: 337,
                                                                as_str(): "x",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 338,
                                                            end: 339,
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
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 340,
                                                                            end: 346,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 346,
                                                                                end: 348,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 348,
                                                                                    end: 350,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: Some(
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 350,
                                                                                            end: 352,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    GenericArgs {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 352,
                                                                                                    end: 353,
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
                                                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 353,
                                                                                                                            end: 356,
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
                                                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 356,
                                                                                                                end: 357,
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
                                                                                                                        src (ptr): 0x00007fb0ed8e1980,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 358,
                                                                                                                        end: 367,
                                                                                                                        as_str(): "SaleError",
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
                                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 367,
                                                                                                    end: 368,
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
                                                        args: Parens {
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
                                                                    Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 369,
                                                                                    end: 370,
                                                                                    as_str(): "5",
                                                                                },
                                                                                parsed: 5,
                                                                                ty_opt: Some(
                                                                                    (
                                                                                        U64,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 370,
                                                                                            end: 373,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 368,
                                                                end: 374,
                                                                as_str(): "(5u64)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 374,
                                                            end: 375,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 380,
                                                            end: 383,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 384,
                                                                    end: 387,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 388,
                                                                end: 389,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 390,
                                                            end: 391,
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
                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 392,
                                                                            end: 398,
                                                                            as_str(): "Result",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 398,
                                                                                end: 400,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                    ),
                                                                                    start: 400,
                                                                                    end: 403,
                                                                                    as_str(): "Err",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            generics_opt: Some(
                                                                                (
                                                                                    DoubleColonToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 403,
                                                                                            end: 405,
                                                                                            as_str(): "::",
                                                                                        },
                                                                                    },
                                                                                    GenericArgs {
                                                                                        parameters: AngleBrackets {
                                                                                            open_angle_bracket_token: OpenAngleBracketToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 405,
                                                                                                    end: 406,
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
                                                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 406,
                                                                                                                            end: 409,
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
                                                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                                ),
                                                                                                                start: 409,
                                                                                                                end: 410,
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
                                                                                                                        src (ptr): 0x00007fb0ed8e1980,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 411,
                                                                                                                        end: 420,
                                                                                                                        as_str(): "SaleError",
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
                                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 420,
                                                                                                    end: 421,
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
                                                                                            src (ptr): 0x00007fb0ed8e1980,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                            ),
                                                                                            start: 422,
                                                                                            end: 431,
                                                                                            as_str(): "SaleError",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [
                                                                                    (
                                                                                        DoubleColonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                ),
                                                                                                start: 431,
                                                                                                end: 433,
                                                                                                as_str(): "::",
                                                                                            },
                                                                                        },
                                                                                        PathExprSegment {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 433,
                                                                                                    end: 451,
                                                                                                    as_str(): "NotEnoughInventory",
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
                                                                                        String(
                                                                                            LitString {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb0ed8e1980,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                                    ),
                                                                                                    start: 452,
                                                                                                    end: 457,
                                                                                                    as_str(): "\"foo\"",
                                                                                                },
                                                                                                parsed: "foo",
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 451,
                                                                                end: 458,
                                                                                as_str(): "(\"foo\")",
                                                                            },
                                                                        },
                                                                    },
                                                                ),
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 421,
                                                                end: 459,
                                                                as_str(): "(SaleError::NotEnoughInventory(\"foo\"))",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 459,
                                                            end: 460,
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
                                                                src (ptr): 0x00007fb0ed8e1980,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                ),
                                                                start: 496,
                                                                end: 497,
                                                                as_str(): "y",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    reassignment_op: ReassignmentOp {
                                                        variant: Equals,
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 498,
                                                            end: 499,
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
                                                                        src (ptr): 0x00007fb0ed8e1980,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 500,
                                                                        end: 501,
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
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 501,
                                                            end: 502,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: Some(
                                            Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fb0ed8e1980,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                                            ),
                                                            start: 507,
                                                            end: 508,
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
                                        src (ptr): 0x00007fb0ed8e1980,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRREsxqm/enum_type_inference/src/main.sw",
                                        ),
                                        start: 326,
                                        end: 510,
                                        as_str(): "{\n    let x = Result::Ok::<u64, SaleError>(5u64);\n    let mut y = Result::Err::<u64, SaleError>(SaleError::NotEnoughInventory(\"foo\"));\n    // should be the same type\n    y = x;\n    5\n}",
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
