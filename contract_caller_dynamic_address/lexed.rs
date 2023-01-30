Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fb110b1a180,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                            ),
                            start: 1,
                            end: 7,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fb110b1a180,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                        ),
                        start: 7,
                        end: 8,
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
                                            src (ptr): 0x00007fb110b1a180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                            ),
                                            start: 11,
                                            end: 13,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb110b1a180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                            ),
                                            start: 14,
                                            end: 18,
                                            as_str(): "main",
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
                                                                    src (ptr): 0x00007fb110b1a180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                    ),
                                                                    start: 19,
                                                                    end: 23,
                                                                    as_str(): "addr",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        colon_token: ColonToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 23,
                                                                end: 24,
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
                                                                            src (ptr): 0x00007fb110b1a180,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                            ),
                                                                            start: 25,
                                                                            end: 29,
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
                                                ),
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb110b1a180,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                            ),
                                            start: 18,
                                            end: 30,
                                            as_str(): "(addr: b256)",
                                        },
                                    },
                                    return_type_opt: Some(
                                        (
                                            RightArrowToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb110b1a180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                    ),
                                                    start: 31,
                                                    end: 33,
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
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 34,
                                                                end: 37,
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
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 42,
                                                            end: 45,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 46,
                                                                end: 52,
                                                                as_str(): "caller",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110b1a180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                    ),
                                                                    start: 52,
                                                                    end: 53,
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
                                                                                src (ptr): 0x00007fb110b1a180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                ),
                                                                                start: 54,
                                                                                end: 68,
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
                                                                                                src (ptr): 0x00007fb110b1a180,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                                ),
                                                                                                start: 68,
                                                                                                end: 69,
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
                                                                                                                    src (ptr): 0x00007fb110b1a180,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 69,
                                                                                                                    end: 76,
                                                                                                                    as_str(): "SomeAbi",
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
                                                                                                src (ptr): 0x00007fb110b1a180,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                                ),
                                                                                                start: 76,
                                                                                                end: 77,
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
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 78,
                                                            end: 79,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: AbiCast {
                                                        abi_token: AbiToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 80,
                                                                end: 83,
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
                                                                                src (ptr): 0x00007fb110b1a180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                ),
                                                                                start: 84,
                                                                                end: 91,
                                                                                as_str(): "SomeAbi",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                                comma_token: CommaToken {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110b1a180,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                        ),
                                                                        start: 91,
                                                                        end: 92,
                                                                        as_str(): ",",
                                                                    },
                                                                },
                                                                address: Path(
                                                                    PathExpr {
                                                                        root_opt: None,
                                                                        prefix: PathExprSegment {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb110b1a180,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                    ),
                                                                                    start: 93,
                                                                                    end: 97,
                                                                                    as_str(): "addr",
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
                                                            span: Span {
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 83,
                                                                end: 98,
                                                                as_str(): "(SomeAbi, addr)",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 98,
                                                            end: 99,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 193,
                                                            end: 196,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Wildcard {
                                                        underscore_token: UnderscoreToken {
                                                            span: Span {
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 197,
                                                                end: 198,
                                                                as_str(): "_",
                                                            },
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 199,
                                                            end: 200,
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
                                                                            src (ptr): 0x00007fb110b1a180,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                            ),
                                                                            start: 201,
                                                                            end: 207,
                                                                            as_str(): "caller",
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
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 207,
                                                                end: 208,
                                                                as_str(): ".",
                                                            },
                                                        },
                                                        path_seg: PathExprSegment {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110b1a180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                    ),
                                                                    start: 208,
                                                                    end: 211,
                                                                    as_str(): "baz",
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
                                                                src (ptr): 0x00007fb110b1a180,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                ),
                                                                start: 211,
                                                                end: 213,
                                                                as_str(): "()",
                                                            },
                                                        },
                                                    },
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 213,
                                                            end: 214,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: Return {
                                                    return_token: ReturnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 217,
                                                            end: 223,
                                                            as_str(): "return",
                                                        },
                                                    },
                                                    expr_opt: Some(
                                                        Literal(
                                                            Int(
                                                                LitInt {
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb110b1a180,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                        ),
                                                                        start: 224,
                                                                        end: 226,
                                                                        as_str(): "42",
                                                                    },
                                                                    parsed: 42,
                                                                    ty_opt: None,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
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
                                        src (ptr): 0x00007fb110b1a180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                        ),
                                        start: 38,
                                        end: 229,
                                        as_str(): "{\n  let caller: ContractCaller<SomeAbi> = abi(SomeAbi, addr);\n  // this should revert since we don't have the script data being passed in to the harness\n  let _ = caller.baz();\n  return 42;\n}",
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
                                        src (ptr): 0x00007fb110b1a180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                        ),
                                        start: 231,
                                        end: 234,
                                        as_str(): "abi",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb110b1a180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                        ),
                                        start: 235,
                                        end: 242,
                                        as_str(): "SomeAbi",
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
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 247,
                                                            end: 249,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 250,
                                                            end: 253,
                                                            as_str(): "baz",
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
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
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
                                                                    src (ptr): 0x00007fb110b1a180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
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
                                                                                src (ptr): 0x00007fb110b1a180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                ),
                                                                                start: 259,
                                                                                end: 262,
                                                                                as_str(): "u32",
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
                                                    src (ptr): 0x00007fb110b1a180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                    ),
                                                    start: 262,
                                                    end: 263,
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
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 266,
                                                            end: 268,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 269,
                                                            end: 273,
                                                            as_str(): "quux",
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
                                                            src (ptr): 0x00007fb110b1a180,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                            ),
                                                            start: 273,
                                                            end: 275,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                    return_type_opt: Some(
                                                        (
                                                            RightArrowToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fb110b1a180,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                    ),
                                                                    start: 276,
                                                                    end: 278,
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
                                                                                src (ptr): 0x00007fb110b1a180,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                                                ),
                                                                                start: 279,
                                                                                end: 282,
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
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fb110b1a180,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                                    ),
                                                    start: 282,
                                                    end: 283,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb110b1a180,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqU3QUZ/contract_caller_dynamic_address/src/main.sw",
                                        ),
                                        start: 243,
                                        end: 285,
                                        as_str(): "{\n  fn baz() -> u32;\n  fn quux() -> u64;\n}",
                                    },
                                },
                                abi_defs_opt: None,
                            },
                        ),
                    },
                ],
            },
            submodules: [],
        },
    },
)
