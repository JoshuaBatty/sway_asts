Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0c6ce4c30,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 9,
                                        end: 13,
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 14,
                                                        end: 18,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 18,
                                                            end: 20,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 20,
                                                                end: 23,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 23,
                                                            end: 25,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 25,
                                                                end: 27,
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
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 28,
                                                end: 31,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Array(
                                    SquareBrackets {
                                        inner: TyArrayDescriptor {
                                            ty: Path(
                                                PathType {
                                                    root_opt: None,
                                                    prefix: PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
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
                                            semicolon_token: SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 36,
                                                    end: 37,
                                                    as_str(): ";",
                                                },
                                            },
                                            length: Literal(
                                                Int(
                                                    LitInt {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 38,
                                                            end: 39,
                                                            as_str(): "2",
                                                        },
                                                        parsed: 2,
                                                        ty_opt: None,
                                                    },
                                                ),
                                            ),
                                        },
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 32,
                                            end: 40,
                                            as_str(): "[u64; 2]",
                                        },
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 47,
                                                            end: 49,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 50,
                                                            end: 52,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 53,
                                                                    end: 57,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 57,
                                                                            end: 58,
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
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 59,
                                                                                            end: 64,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 64,
                                                                                        end: 65,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 66,
                                                                                                    end: 70,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 52,
                                                            end: 71,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 72,
                                                                    end: 74,
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
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 75,
                                                                                end: 79,
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
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 90,
                                                                            end: 93,
                                                                            as_str(): "let",
                                                                        },
                                                                    },
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: Some(
                                                                            MutToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 94,
                                                                                    end: 97,
                                                                                    as_str(): "mut",
                                                                                },
                                                                            },
                                                                        ),
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 98,
                                                                                end: 99,
                                                                                as_str(): "i",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    ty_opt: None,
                                                                    eq_token: EqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 100,
                                                                            end: 101,
                                                                            as_str(): "=",
                                                                        },
                                                                    },
                                                                    expr: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 102,
                                                                                    end: 103,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    semicolon_token: SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 103,
                                                                            end: 104,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                            Expr {
                                                                expr: While {
                                                                    while_token: WhileToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 113,
                                                                            end: 118,
                                                                            as_str(): "while",
                                                                        },
                                                                    },
                                                                    condition: LessThan {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 119,
                                                                                            end: 120,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        less_than_token: LessThanToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 121,
                                                                                end: 122,
                                                                                as_str(): "<",
                                                                            },
                                                                        },
                                                                        rhs: Literal(
                                                                            Int(
                                                                                LitInt {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 123,
                                                                                        end: 124,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                    parsed: 2,
                                                                                    ty_opt: None,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [
                                                                                Expr {
                                                                                    expr: If(
                                                                                        IfExpr {
                                                                                            if_token: IfToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 139,
                                                                                                    end: 141,
                                                                                                    as_str(): "if",
                                                                                                },
                                                                                            },
                                                                                            condition: Expr(
                                                                                                NotEqual {
                                                                                                    lhs: Index {
                                                                                                        target: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 142,
                                                                                                                            end: 146,
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
                                                                                                        arg: SquareBrackets {
                                                                                                            inner: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 147,
                                                                                                                                end: 148,
                                                                                                                                as_str(): "i",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 146,
                                                                                                                end: 149,
                                                                                                                as_str(): "[i]",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                    bang_eq_token: BangEqToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 150,
                                                                                                            end: 152,
                                                                                                            as_str(): "!=",
                                                                                                        },
                                                                                                    },
                                                                                                    rhs: Index {
                                                                                                        target: Path(
                                                                                                            PathExpr {
                                                                                                                root_opt: None,
                                                                                                                prefix: PathExprSegment {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 153,
                                                                                                                            end: 158,
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
                                                                                                        arg: SquareBrackets {
                                                                                                            inner: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 159,
                                                                                                                                end: 160,
                                                                                                                                as_str(): "i",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        generics_opt: None,
                                                                                                                    },
                                                                                                                    suffix: [],
                                                                                                                    incomplete_suffix: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 158,
                                                                                                                end: 161,
                                                                                                                as_str(): "[i]",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            then_block: Braces {
                                                                                                inner: CodeBlockContents {
                                                                                                    statements: [
                                                                                                        Expr {
                                                                                                            expr: Return {
                                                                                                                return_token: ReturnToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 180,
                                                                                                                        end: 186,
                                                                                                                        as_str(): "return",
                                                                                                                    },
                                                                                                                },
                                                                                                                expr_opt: Some(
                                                                                                                    Literal(
                                                                                                                        Bool(
                                                                                                                            LitBool {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 187,
                                                                                                                                    end: 192,
                                                                                                                                    as_str(): "false",
                                                                                                                                },
                                                                                                                                kind: False,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                ),
                                                                                                            },
                                                                                                            semicolon_token_opt: Some(
                                                                                                                SemicolonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 192,
                                                                                                                        end: 193,
                                                                                                                        as_str(): ";",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                    ],
                                                                                                    final_expr_opt: None,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 162,
                                                                                                    end: 207,
                                                                                                    as_str(): "{\n                return false;\n            }",
                                                                                                },
                                                                                            },
                                                                                            else_opt: None,
                                                                                        },
                                                                                    ),
                                                                                    semicolon_token_opt: None,
                                                                                },
                                                                                Expr {
                                                                                    expr: Reassignment {
                                                                                        assignable: Var(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 220,
                                                                                                    end: 221,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        reassignment_op: ReassignmentOp {
                                                                                            variant: AddEquals,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 222,
                                                                                                end: 224,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        expr: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 225,
                                                                                                        end: 226,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                    parsed: 1,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    semicolon_token_opt: Some(
                                                                                        SemicolonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 226,
                                                                                                end: 227,
                                                                                                as_str(): ";",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 125,
                                                                            end: 237,
                                                                            as_str(): "{\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }",
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token_opt: None,
                                                            },
                                                        ],
                                                        final_expr_opt: Some(
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 246,
                                                                            end: 250,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 80,
                                                        end: 256,
                                                        as_str(): "{\n        let mut i = 0;\n        while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 258,
                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        let mut i = 0;\n        while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 260,
                                        end: 264,
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 265,
                                                        end: 269,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 269,
                                                            end: 271,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 271,
                                                                end: 274,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 276,
                                                            as_str(): "::",
                                                        },
                                                    },
                                                    PathTypeSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 276,
                                                                end: 278,
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
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 279,
                                                end: 282,
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
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 283,
                                                    end: 286,
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
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 286,
                                                                    end: 287,
                                                                    as_str(): "<",
                                                                },
                                                            },
                                                            inner: Punctuated {
                                                                value_separator_pairs: [],
                                                                final_value_opt: Some(
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 288,
                                                                                                    end: 291,
                                                                                                    as_str(): "u64",
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
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 291,
                                                                                        end: 292,
                                                                                        as_str(): ";",
                                                                                    },
                                                                                },
                                                                                length: Literal(
                                                                                    Int(
                                                                                        LitInt {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 293,
                                                                                                end: 294,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                            parsed: 2,
                                                                                            ty_opt: None,
                                                                                        },
                                                                                    ),
                                                                                ),
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 287,
                                                                                end: 295,
                                                                                as_str(): "[u64; 2]",
                                                                            },
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                            close_angle_bracket_token: CloseAngleBracketToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 295,
                                                                    end: 296,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 303,
                                                            end: 305,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 306,
                                                            end: 308,
                                                            as_str(): "eq",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 309,
                                                                    end: 313,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: Some(
                                                                (
                                                                    CommaToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 313,
                                                                            end: 314,
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
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 315,
                                                                                            end: 320,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                colon_token: ColonToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 320,
                                                                                        end: 321,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 322,
                                                                                                    end: 326,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 308,
                                                            end: 327,
                                                            as_str(): "(self, other: Self)",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 328,
                                                                    end: 330,
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
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 331,
                                                                                end: 335,
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
                                                            Expr {
                                                                expr: If(
                                                                    IfExpr {
                                                                        if_token: IfToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 346,
                                                                                end: 348,
                                                                                as_str(): "if",
                                                                            },
                                                                        },
                                                                        condition: Expr(
                                                                            NotEqual {
                                                                                lhs: MethodCall {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
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
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 353,
                                                                                            end: 354,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    path_seg: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 354,
                                                                                                end: 357,
                                                                                                as_str(): "len",
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
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 357,
                                                                                            end: 359,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                bang_eq_token: BangEqToken {
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 360,
                                                                                        end: 362,
                                                                                        as_str(): "!=",
                                                                                    },
                                                                                },
                                                                                rhs: MethodCall {
                                                                                    target: Path(
                                                                                        PathExpr {
                                                                                            root_opt: None,
                                                                                            prefix: PathExprSegment {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 363,
                                                                                                        end: 368,
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
                                                                                    dot_token: DotToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 368,
                                                                                            end: 369,
                                                                                            as_str(): ".",
                                                                                        },
                                                                                    },
                                                                                    path_seg: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 369,
                                                                                                end: 372,
                                                                                                as_str(): "len",
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
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 372,
                                                                                            end: 374,
                                                                                            as_str(): "()",
                                                                                        },
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                        then_block: Braces {
                                                                            inner: CodeBlockContents {
                                                                                statements: [
                                                                                    Expr {
                                                                                        expr: Return {
                                                                                            return_token: ReturnToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 389,
                                                                                                    end: 395,
                                                                                                    as_str(): "return",
                                                                                                },
                                                                                            },
                                                                                            expr_opt: Some(
                                                                                                Literal(
                                                                                                    Bool(
                                                                                                        LitBool {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 396,
                                                                                                                end: 401,
                                                                                                                as_str(): "false",
                                                                                                            },
                                                                                                            kind: False,
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            ),
                                                                                        },
                                                                                        semicolon_token_opt: Some(
                                                                                            SemicolonToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 401,
                                                                                                    end: 402,
                                                                                                    as_str(): ";",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                                final_expr_opt: None,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 375,
                                                                                end: 412,
                                                                                as_str(): "{\n            return false;\n        }",
                                                                            },
                                                                        },
                                                                        else_opt: None,
                                                                    },
                                                                ),
                                                                semicolon_token_opt: None,
                                                            },
                                                            Let(
                                                                StatementLet {
                                                                    let_token: LetToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 421,
                                                                            end: 424,
                                                                            as_str(): "let",
                                                                        },
                                                                    },
                                                                    pattern: Var {
                                                                        reference: None,
                                                                        mutable: Some(
                                                                            MutToken {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 425,
                                                                                    end: 428,
                                                                                    as_str(): "mut",
                                                                                },
                                                                            },
                                                                        ),
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 429,
                                                                                end: 430,
                                                                                as_str(): "i",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                    },
                                                                    ty_opt: None,
                                                                    eq_token: EqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 431,
                                                                            end: 432,
                                                                            as_str(): "=",
                                                                        },
                                                                    },
                                                                    expr: Literal(
                                                                        Int(
                                                                            LitInt {
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 433,
                                                                                    end: 434,
                                                                                    as_str(): "0",
                                                                                },
                                                                                parsed: 0,
                                                                                ty_opt: None,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    semicolon_token: SemicolonToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 434,
                                                                            end: 435,
                                                                            as_str(): ";",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                            Expr {
                                                                expr: While {
                                                                    while_token: WhileToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 444,
                                                                            end: 449,
                                                                            as_str(): "while",
                                                                        },
                                                                    },
                                                                    condition: LessThan {
                                                                        lhs: Path(
                                                                            PathExpr {
                                                                                root_opt: None,
                                                                                prefix: PathExprSegment {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 450,
                                                                                            end: 451,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    generics_opt: None,
                                                                                },
                                                                                suffix: [],
                                                                                incomplete_suffix: false,
                                                                            },
                                                                        ),
                                                                        less_than_token: LessThanToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 452,
                                                                                end: 453,
                                                                                as_str(): "<",
                                                                            },
                                                                        },
                                                                        rhs: MethodCall {
                                                                            target: Path(
                                                                                PathExpr {
                                                                                    root_opt: None,
                                                                                    prefix: PathExprSegment {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 454,
                                                                                                end: 458,
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
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 458,
                                                                                    end: 459,
                                                                                    as_str(): ".",
                                                                                },
                                                                            },
                                                                            path_seg: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 459,
                                                                                        end: 462,
                                                                                        as_str(): "len",
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
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 462,
                                                                                    end: 464,
                                                                                    as_str(): "()",
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    block: Braces {
                                                                        inner: CodeBlockContents {
                                                                            statements: [
                                                                                Expr {
                                                                                    expr: If(
                                                                                        IfExpr {
                                                                                            if_token: IfToken {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 479,
                                                                                                    end: 481,
                                                                                                    as_str(): "if",
                                                                                                },
                                                                                            },
                                                                                            condition: Expr(
                                                                                                NotEqual {
                                                                                                    lhs: MethodCall {
                                                                                                        target: MethodCall {
                                                                                                            target: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
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
                                                                                                            dot_token: DotToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 486,
                                                                                                                    end: 487,
                                                                                                                    as_str(): ".",
                                                                                                                },
                                                                                                            },
                                                                                                            path_seg: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 487,
                                                                                                                        end: 490,
                                                                                                                        as_str(): "get",
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
                                                                                                                        Path(
                                                                                                                            PathExpr {
                                                                                                                                root_opt: None,
                                                                                                                                prefix: PathExprSegment {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 491,
                                                                                                                                            end: 492,
                                                                                                                                            as_str(): "i",
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
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 490,
                                                                                                                    end: 493,
                                                                                                                    as_str(): "(i)",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        dot_token: DotToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 493,
                                                                                                                end: 494,
                                                                                                                as_str(): ".",
                                                                                                            },
                                                                                                        },
                                                                                                        path_seg: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 494,
                                                                                                                    end: 500,
                                                                                                                    as_str(): "unwrap",
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
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 500,
                                                                                                                end: 502,
                                                                                                                as_str(): "()",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                    bang_eq_token: BangEqToken {
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                            ),
                                                                                                            start: 503,
                                                                                                            end: 505,
                                                                                                            as_str(): "!=",
                                                                                                        },
                                                                                                    },
                                                                                                    rhs: MethodCall {
                                                                                                        target: MethodCall {
                                                                                                            target: Path(
                                                                                                                PathExpr {
                                                                                                                    root_opt: None,
                                                                                                                    prefix: PathExprSegment {
                                                                                                                        name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 506,
                                                                                                                                end: 511,
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
                                                                                                            dot_token: DotToken {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 511,
                                                                                                                    end: 512,
                                                                                                                    as_str(): ".",
                                                                                                                },
                                                                                                            },
                                                                                                            path_seg: PathExprSegment {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 512,
                                                                                                                        end: 515,
                                                                                                                        as_str(): "get",
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
                                                                                                                        Path(
                                                                                                                            PathExpr {
                                                                                                                                root_opt: None,
                                                                                                                                prefix: PathExprSegment {
                                                                                                                                    name: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 516,
                                                                                                                                            end: 517,
                                                                                                                                            as_str(): "i",
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
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 515,
                                                                                                                    end: 518,
                                                                                                                    as_str(): "(i)",
                                                                                                                },
                                                                                                            },
                                                                                                        },
                                                                                                        dot_token: DotToken {
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 518,
                                                                                                                end: 519,
                                                                                                                as_str(): ".",
                                                                                                            },
                                                                                                        },
                                                                                                        path_seg: PathExprSegment {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 519,
                                                                                                                    end: 525,
                                                                                                                    as_str(): "unwrap",
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
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 525,
                                                                                                                end: 527,
                                                                                                                as_str(): "()",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                            then_block: Braces {
                                                                                                inner: CodeBlockContents {
                                                                                                    statements: [
                                                                                                        Expr {
                                                                                                            expr: Return {
                                                                                                                return_token: ReturnToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 546,
                                                                                                                        end: 552,
                                                                                                                        as_str(): "return",
                                                                                                                    },
                                                                                                                },
                                                                                                                expr_opt: Some(
                                                                                                                    Literal(
                                                                                                                        Bool(
                                                                                                                            LitBool {
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 553,
                                                                                                                                    end: 558,
                                                                                                                                    as_str(): "false",
                                                                                                                                },
                                                                                                                                kind: False,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                ),
                                                                                                            },
                                                                                                            semicolon_token_opt: Some(
                                                                                                                SemicolonToken {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 558,
                                                                                                                        end: 559,
                                                                                                                        as_str(): ";",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                    ],
                                                                                                    final_expr_opt: None,
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 528,
                                                                                                    end: 573,
                                                                                                    as_str(): "{\n                return false;\n            }",
                                                                                                },
                                                                                            },
                                                                                            else_opt: None,
                                                                                        },
                                                                                    ),
                                                                                    semicolon_token_opt: None,
                                                                                },
                                                                                Expr {
                                                                                    expr: Reassignment {
                                                                                        assignable: Var(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 586,
                                                                                                    end: 587,
                                                                                                    as_str(): "i",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        reassignment_op: ReassignmentOp {
                                                                                            variant: AddEquals,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 588,
                                                                                                end: 590,
                                                                                                as_str(): "+=",
                                                                                            },
                                                                                        },
                                                                                        expr: Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 591,
                                                                                                        end: 592,
                                                                                                        as_str(): "1",
                                                                                                    },
                                                                                                    parsed: 1,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    semicolon_token_opt: Some(
                                                                                        SemicolonToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 592,
                                                                                                end: 593,
                                                                                                as_str(): ";",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                            ],
                                                                            final_expr_opt: None,
                                                                        },
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 465,
                                                                            end: 603,
                                                                            as_str(): "{\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }",
                                                                        },
                                                                    },
                                                                },
                                                                semicolon_token_opt: None,
                                                            },
                                                        ],
                                                        final_expr_opt: Some(
                                                            Literal(
                                                                Bool(
                                                                    LitBool {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 612,
                                                                            end: 616,
                                                                            as_str(): "true",
                                                                        },
                                                                        kind: True,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 336,
                                                        end: 622,
                                                        as_str(): "{\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 297,
                                        end: 624,
                                        as_str(): "{\n    fn eq(self, other: Self) -> bool {\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
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
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 626,
                                            end: 628,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 629,
                                            end: 636,
                                            as_str(): "tester1",
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
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 637,
                                                                    end: 640,
                                                                    as_str(): "arg",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 640,
                                                                end: 641,
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
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 642,
                                                                            end: 645,
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
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 645,
                                                                                            end: 646,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                    },
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
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
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 647,
                                                                                                                            end: 650,
                                                                                                                            as_str(): "u64",
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
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 650,
                                                                                                                end: 651,
                                                                                                                as_str(): ";",
                                                                                                            },
                                                                                                        },
                                                                                                        length: Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 652,
                                                                                                                        end: 653,
                                                                                                                        as_str(): "2",
                                                                                                                    },
                                                                                                                    parsed: 2,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 646,
                                                                                                        end: 654,
                                                                                                        as_str(): "[u64; 2]",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 654,
                                                                                            end: 655,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 636,
                                            end: 656,
                                            as_str(): "(arg: Vec<[u64; 2]>)",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 663,
                                                            end: 666,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 667,
                                                                    end: 670,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 671,
                                                                end: 679,
                                                                as_str(): "expected",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 680,
                                                            end: 681,
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
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 682,
                                                                            end: 685,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 685,
                                                                                end: 687,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 687,
                                                                                    end: 690,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 690,
                                                                end: 692,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 692,
                                                            end: 693,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 698,
                                                                        end: 706,
                                                                        as_str(): "expected",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 706,
                                                            end: 707,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 707,
                                                                end: 711,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 713,
                                                                                                        end: 714,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 714,
                                                                                                end: 715,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 716,
                                                                                                    end: 717,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 712,
                                                                            end: 718,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 711,
                                                            end: 719,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 719,
                                                            end: 720,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 725,
                                                                        end: 733,
                                                                        as_str(): "expected",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 733,
                                                            end: 734,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 734,
                                                                end: 738,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 740,
                                                                                                        end: 741,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 741,
                                                                                                end: 742,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 743,
                                                                                                    end: 744,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 739,
                                                                            end: 745,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 738,
                                                            end: 746,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 746,
                                                            end: 747,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 753,
                                                                        end: 759,
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
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 760,
                                                                                        end: 763,
                                                                                        as_str(): "arg",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    double_eq_token: DoubleEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 764,
                                                                            end: 766,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    rhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 767,
                                                                                        end: 775,
                                                                                        as_str(): "expected",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 759,
                                                            end: 776,
                                                            as_str(): "(arg == expected)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 776,
                                                            end: 777,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 657,
                                        end: 779,
                                        as_str(): "{\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg == expected);\n}",
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
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 781,
                                            end: 783,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 784,
                                            end: 791,
                                            as_str(): "tester2",
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
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 792,
                                                                    end: 795,
                                                                    as_str(): "arg",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 795,
                                                                end: 796,
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
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 797,
                                                                            end: 800,
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
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 800,
                                                                                            end: 801,
                                                                                            as_str(): "<",
                                                                                        },
                                                                                    },
                                                                                    inner: Punctuated {
                                                                                        value_separator_pairs: [],
                                                                                        final_value_opt: Some(
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
                                                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 802,
                                                                                                                            end: 805,
                                                                                                                            as_str(): "u64",
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
                                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                ),
                                                                                                                start: 805,
                                                                                                                end: 806,
                                                                                                                as_str(): ";",
                                                                                                            },
                                                                                                        },
                                                                                                        length: Literal(
                                                                                                            Int(
                                                                                                                LitInt {
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 807,
                                                                                                                        end: 808,
                                                                                                                        as_str(): "2",
                                                                                                                    },
                                                                                                                    parsed: 2,
                                                                                                                    ty_opt: None,
                                                                                                                },
                                                                                                            ),
                                                                                                        ),
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 801,
                                                                                                        end: 809,
                                                                                                        as_str(): "[u64; 2]",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                    },
                                                                                    close_angle_bracket_token: CloseAngleBracketToken {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                            ),
                                                                                            start: 809,
                                                                                            end: 810,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 791,
                                            end: 811,
                                            as_str(): "(arg: Vec<[u64; 2]>)",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 818,
                                                            end: 821,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 822,
                                                                    end: 825,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 826,
                                                                end: 834,
                                                                as_str(): "expected",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 835,
                                                            end: 836,
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
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 837,
                                                                            end: 840,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 840,
                                                                                end: 842,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 842,
                                                                                    end: 845,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 845,
                                                                end: 847,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 847,
                                                            end: 848,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 853,
                                                                        end: 861,
                                                                        as_str(): "expected",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 861,
                                                            end: 862,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 862,
                                                                end: 866,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 868,
                                                                                                        end: 869,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 869,
                                                                                                end: 870,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 871,
                                                                                                    end: 872,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 867,
                                                                            end: 873,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 866,
                                                            end: 874,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 874,
                                                            end: 875,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 880,
                                                                        end: 888,
                                                                        as_str(): "expected",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 888,
                                                            end: 889,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 889,
                                                                end: 893,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 895,
                                                                                                        end: 896,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 896,
                                                                                                end: 897,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 898,
                                                                                                    end: 899,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 894,
                                                                            end: 900,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 893,
                                                            end: 901,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 901,
                                                            end: 902,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 908,
                                                                        end: 914,
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
                                                                NotEqual {
                                                                    lhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 915,
                                                                                        end: 918,
                                                                                        as_str(): "arg",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                generics_opt: None,
                                                                            },
                                                                            suffix: [],
                                                                            incomplete_suffix: false,
                                                                        },
                                                                    ),
                                                                    bang_eq_token: BangEqToken {
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 919,
                                                                            end: 921,
                                                                            as_str(): "!=",
                                                                        },
                                                                    },
                                                                    rhs: Path(
                                                                        PathExpr {
                                                                            root_opt: None,
                                                                            prefix: PathExprSegment {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                        ),
                                                                                        start: 922,
                                                                                        end: 930,
                                                                                        as_str(): "expected",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 914,
                                                            end: 931,
                                                            as_str(): "(arg != expected)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 931,
                                                            end: 932,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 812,
                                        end: 934,
                                        as_str(): "{\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg != expected);\n}",
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
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 936,
                                            end: 938,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 939,
                                            end: 943,
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
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 943,
                                            end: 945,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 946,
                                                    end: 948,
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
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 949,
                                                                end: 952,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 958,
                                                            end: 961,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 962,
                                                                    end: 965,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 966,
                                                                end: 970,
                                                                as_str(): "arg1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 971,
                                                            end: 972,
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
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 973,
                                                                            end: 976,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 976,
                                                                                end: 978,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 978,
                                                                                    end: 981,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 981,
                                                                end: 983,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 983,
                                                            end: 984,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 987,
                                                                        end: 991,
                                                                        as_str(): "arg1",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 991,
                                                            end: 992,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 992,
                                                                end: 996,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 998,
                                                                                                        end: 999,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 999,
                                                                                                end: 1000,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1001,
                                                                                                    end: 1002,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 997,
                                                                            end: 1003,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 996,
                                                            end: 1004,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1004,
                                                            end: 1005,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 1008,
                                                                        end: 1012,
                                                                        as_str(): "arg1",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1012,
                                                            end: 1013,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1013,
                                                                end: 1017,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1019,
                                                                                                        end: 1020,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 1020,
                                                                                                end: 1021,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1022,
                                                                                                    end: 1023,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1018,
                                                                            end: 1024,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1017,
                                                            end: 1025,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1025,
                                                            end: 1026,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 1029,
                                                                        end: 1036,
                                                                        as_str(): "tester1",
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
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 1037,
                                                                                    end: 1041,
                                                                                    as_str(): "arg1",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1036,
                                                            end: 1042,
                                                            as_str(): "(arg1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1042,
                                                            end: 1043,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1047,
                                                            end: 1050,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: Some(
                                                            MutToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 1051,
                                                                    end: 1054,
                                                                    as_str(): "mut",
                                                                },
                                                            },
                                                        ),
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1055,
                                                                end: 1059,
                                                                as_str(): "arg2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1060,
                                                            end: 1061,
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
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1062,
                                                                            end: 1065,
                                                                            as_str(): "Vec",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    generics_opt: None,
                                                                },
                                                                suffix: [
                                                                    (
                                                                        DoubleColonToken {
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                ),
                                                                                start: 1065,
                                                                                end: 1067,
                                                                                as_str(): "::",
                                                                            },
                                                                        },
                                                                        PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 1067,
                                                                                    end: 1070,
                                                                                    as_str(): "new",
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
                                                                final_value_opt: None,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1070,
                                                                end: 1072,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1072,
                                                            end: 1073,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 1076,
                                                                        end: 1080,
                                                                        as_str(): "arg2",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1080,
                                                            end: 1081,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1081,
                                                                end: 1085,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1087,
                                                                                                        end: 1088,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 1088,
                                                                                                end: 1089,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1090,
                                                                                                    end: 1091,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1086,
                                                                            end: 1092,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1085,
                                                            end: 1093,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1093,
                                                            end: 1094,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 1097,
                                                                        end: 1101,
                                                                        as_str(): "arg2",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1101,
                                                            end: 1102,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1102,
                                                                end: 1106,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1108,
                                                                                                        end: 1109,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 1109,
                                                                                                end: 1110,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1111,
                                                                                                    end: 1112,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                                parsed: 2,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1107,
                                                                            end: 1113,
                                                                            as_str(): "[0, 2]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1106,
                                                            end: 1114,
                                                            as_str(): "([0, 2])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1114,
                                                            end: 1115,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 1118,
                                                                        end: 1125,
                                                                        as_str(): "tester2",
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
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 1126,
                                                                                    end: 1130,
                                                                                    as_str(): "arg2",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1125,
                                                            end: 1131,
                                                            as_str(): "(arg2)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1131,
                                                            end: 1132,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 1136,
                                                                        end: 1140,
                                                                        as_str(): "arg1",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1140,
                                                            end: 1141,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 1141,
                                                                end: 1145,
                                                                as_str(): "push",
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
                                                                Array(
                                                                    SquareBrackets {
                                                                        inner: Sequence(
                                                                            Punctuated {
                                                                                value_separator_pairs: [
                                                                                    (
                                                                                        Literal(
                                                                                            Int(
                                                                                                LitInt {
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1147,
                                                                                                        end: 1148,
                                                                                                        as_str(): "0",
                                                                                                    },
                                                                                                    parsed: 0,
                                                                                                    ty_opt: None,
                                                                                                },
                                                                                            ),
                                                                                        ),
                                                                                        CommaToken {
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                ),
                                                                                                start: 1148,
                                                                                                end: 1149,
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
                                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1150,
                                                                                                    end: 1151,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                                parsed: 1,
                                                                                                ty_opt: None,
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                ),
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                            ),
                                                                            start: 1146,
                                                                            end: 1152,
                                                                            as_str(): "[0, 1]",
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1145,
                                                            end: 1153,
                                                            as_str(): "([0, 1])",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1153,
                                                            end: 1154,
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
                                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                        ),
                                                                        start: 1157,
                                                                        end: 1164,
                                                                        as_str(): "tester2",
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
                                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                                    ),
                                                                                    start: 1165,
                                                                                    end: 1169,
                                                                                    as_str(): "arg1",
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1164,
                                                            end: 1170,
                                                            as_str(): "(arg1)",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1170,
                                                            end: 1171,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 1175,
                                                            end: 1176,
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 953,
                                        end: 1178,
                                        as_str(): "{\n\n  let mut arg1 = Vec::new();\n  arg1.push([0, 1]);\n  arg1.push([0, 1]);\n  tester1(arg1);\n\n  let mut arg2 = Vec::new();\n  arg2.push([0, 1]);\n  arg2.push([0, 2]);\n  tester2(arg2);\n\n  arg1.push([0, 1]);\n  tester2(arg1);\n\n  1\n}",
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
