[
    AstNode {
        content: Declaration(
            StructDeclaration(
                StructDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0ef5d6490,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                            ),
                            start: 16,
                            end: 29,
                            as_str(): "GenericStruct",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    fields: [
                        StructField {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 39,
                                    end: 40,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 41,
                                        end: 42,
                                        as_str(): "T",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [],
                                ),
                            },
                            span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 39,
                                end: 42,
                                as_str(): "x:T",
                            },
                            type_span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 41,
                                end: 42,
                                as_str(): "T",
                            },
                        },
                    ],
                    type_parameters: [
                        T: TypeId(31628),
                    ],
                    visibility: Private,
                    span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 9,
                        end: 44,
                        as_str(): "struct GenericStruct<T> {\n    x:T\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 9,
            end: 44,
            as_str(): "struct GenericStruct<T> {\n    x:T\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [
                        T: TypeId(31635),
                        E: TypeId(31636),
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 57,
                                end: 63,
                                as_str(): "Option",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        31631,
                                    ),
                                    initial_type_id: TypeId(
                                        31631,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 64,
                                        end: 76,
                                        as_str(): "Result<T, E>",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 57,
                        end: 77,
                        as_str(): "Option<Result<T, E>>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 91,
                                    end: 100,
                                    as_str(): "transpose",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Public,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: CodeBlock(
                                                    CodeBlock {
                                                        contents: [
                                                            AstNode {
                                                                content: Declaration(
                                                                    VariableDeclaration(
                                                                        VariableDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_1",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 149,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_ascription: Unknown,
                                                                            type_ascription_span: None,
                                                                            body: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 145,
                                                                                            end: 149,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 145,
                                                                                    end: 149,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 139,
                                                                    end: 339,
                                                                    as_str(): "match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }",
                                                                },
                                                            },
                                                            AstNode {
                                                                content: ImplicitReturnExpression(
                                                                    Expression {
                                                                        kind: Match(
                                                                            MatchExpression {
                                                                                value: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_1",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 145,
                                                                                                end: 149,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 145,
                                                                                        end: 149,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: EnumScrutinee {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 162,
                                                                                                            end: 168,
                                                                                                            as_str(): "Option",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 170,
                                                                                                        end: 174,
                                                                                                        as_str(): "Some",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            value: EnumScrutinee {
                                                                                                call_path: CallPath {
                                                                                                    prefixes: [
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 175,
                                                                                                                end: 181,
                                                                                                                as_str(): "Result",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 183,
                                                                                                            end: 185,
                                                                                                            as_str(): "Ok",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                value: Variable {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 186,
                                                                                                            end: 187,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 186,
                                                                                                        end: 187,
                                                                                                        as_str(): "x",
                                                                                                    },
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 175,
                                                                                                    end: 188,
                                                                                                    as_str(): "Result::Ok(x)",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 162,
                                                                                                end: 189,
                                                                                                as_str(): "Option::Some(Result::Ok(x))",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: AmbiguousPathExpression(
                                                                                                AmbiguousPathExpression {
                                                                                                    call_path_binding: TypeBinding {
                                                                                                        inner: CallPath {
                                                                                                            prefixes: [],
                                                                                                            suffix: AmbiguousSuffix {
                                                                                                                before: TypeBinding {
                                                                                                                    inner: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 193,
                                                                                                                            end: 199,
                                                                                                                            as_str(): "Result",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 193,
                                                                                                                        end: 199,
                                                                                                                        as_str(): "Result",
                                                                                                                    },
                                                                                                                },
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 201,
                                                                                                                        end: 203,
                                                                                                                        as_str(): "Ok",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 193,
                                                                                                            end: 203,
                                                                                                            as_str(): "Result::Ok",
                                                                                                        },
                                                                                                    },
                                                                                                    args: [
                                                                                                        Expression {
                                                                                                            kind: AmbiguousPathExpression(
                                                                                                                AmbiguousPathExpression {
                                                                                                                    call_path_binding: TypeBinding {
                                                                                                                        inner: CallPath {
                                                                                                                            prefixes: [],
                                                                                                                            suffix: AmbiguousSuffix {
                                                                                                                                before: TypeBinding {
                                                                                                                                    inner: BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 204,
                                                                                                                                            end: 210,
                                                                                                                                            as_str(): "Option",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                    type_arguments: [],
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 204,
                                                                                                                                        end: 210,
                                                                                                                                        as_str(): "Option",
                                                                                                                                    },
                                                                                                                                },
                                                                                                                                suffix: BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 212,
                                                                                                                                        end: 216,
                                                                                                                                        as_str(): "Some",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            },
                                                                                                                            is_absolute: false,
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 204,
                                                                                                                            end: 216,
                                                                                                                            as_str(): "Option::Some",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    args: [
                                                                                                                        Expression {
                                                                                                                            kind: Variable(
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 217,
                                                                                                                                        end: 218,
                                                                                                                                        as_str(): "x",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 217,
                                                                                                                                end: 218,
                                                                                                                                as_str(): "x",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 204,
                                                                                                                end: 219,
                                                                                                                as_str(): "Option::Some(x)",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 193,
                                                                                                end: 220,
                                                                                                as_str(): "Result::Ok(Option::Some(x))",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 162,
                                                                                            end: 221,
                                                                                            as_str(): "Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: EnumScrutinee {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 232,
                                                                                                            end: 238,
                                                                                                            as_str(): "Option",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 240,
                                                                                                        end: 244,
                                                                                                        as_str(): "Some",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            value: EnumScrutinee {
                                                                                                call_path: CallPath {
                                                                                                    prefixes: [
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 245,
                                                                                                                end: 251,
                                                                                                                as_str(): "Result",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 253,
                                                                                                            end: 256,
                                                                                                            as_str(): "Err",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                value: Variable {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 257,
                                                                                                            end: 258,
                                                                                                            as_str(): "e",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 257,
                                                                                                        end: 258,
                                                                                                        as_str(): "e",
                                                                                                    },
                                                                                                },
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 245,
                                                                                                    end: 259,
                                                                                                    as_str(): "Result::Err(e)",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 232,
                                                                                                end: 260,
                                                                                                as_str(): "Option::Some(Result::Err(e))",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: AmbiguousPathExpression(
                                                                                                AmbiguousPathExpression {
                                                                                                    call_path_binding: TypeBinding {
                                                                                                        inner: CallPath {
                                                                                                            prefixes: [],
                                                                                                            suffix: AmbiguousSuffix {
                                                                                                                before: TypeBinding {
                                                                                                                    inner: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 264,
                                                                                                                            end: 270,
                                                                                                                            as_str(): "Result",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 264,
                                                                                                                        end: 270,
                                                                                                                        as_str(): "Result",
                                                                                                                    },
                                                                                                                },
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 272,
                                                                                                                        end: 275,
                                                                                                                        as_str(): "Err",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 264,
                                                                                                            end: 275,
                                                                                                            as_str(): "Result::Err",
                                                                                                        },
                                                                                                    },
                                                                                                    args: [
                                                                                                        Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 276,
                                                                                                                        end: 277,
                                                                                                                        as_str(): "e",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 276,
                                                                                                                end: 277,
                                                                                                                as_str(): "e",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 264,
                                                                                                end: 278,
                                                                                                as_str(): "Result::Err(e)",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 232,
                                                                                            end: 279,
                                                                                            as_str(): "Option::Some(Result::Err(e)) => Result::Err(e),",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: EnumScrutinee {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 290,
                                                                                                            end: 296,
                                                                                                            as_str(): "Option",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ],
                                                                                                suffix: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 298,
                                                                                                        end: 302,
                                                                                                        as_str(): "None",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                is_absolute: false,
                                                                                            },
                                                                                            value: CatchAll {
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 290,
                                                                                                    end: 302,
                                                                                                    as_str(): "Option::None",
                                                                                                },
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 290,
                                                                                                end: 302,
                                                                                                as_str(): "Option::None",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: AmbiguousPathExpression(
                                                                                                AmbiguousPathExpression {
                                                                                                    call_path_binding: TypeBinding {
                                                                                                        inner: CallPath {
                                                                                                            prefixes: [],
                                                                                                            suffix: AmbiguousSuffix {
                                                                                                                before: TypeBinding {
                                                                                                                    inner: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 306,
                                                                                                                            end: 312,
                                                                                                                            as_str(): "Result",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 306,
                                                                                                                        end: 312,
                                                                                                                        as_str(): "Result",
                                                                                                                    },
                                                                                                                },
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 314,
                                                                                                                        end: 316,
                                                                                                                        as_str(): "Ok",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 306,
                                                                                                            end: 316,
                                                                                                            as_str(): "Result::Ok",
                                                                                                        },
                                                                                                    },
                                                                                                    args: [
                                                                                                        Expression {
                                                                                                            kind: DelineatedPath(
                                                                                                                DelineatedPathExpression {
                                                                                                                    call_path_binding: TypeBinding {
                                                                                                                        inner: CallPath {
                                                                                                                            prefixes: [
                                                                                                                                BaseIdent {
                                                                                                                                    name_override_opt: None,
                                                                                                                                    span: Span {
                                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                        path: Some(
                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                        ),
                                                                                                                                        start: 317,
                                                                                                                                        end: 323,
                                                                                                                                        as_str(): "Option",
                                                                                                                                    },
                                                                                                                                    is_raw_ident: false,
                                                                                                                                },
                                                                                                                            ],
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 325,
                                                                                                                                    end: 329,
                                                                                                                                    as_str(): "None",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            is_absolute: false,
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 325,
                                                                                                                            end: 329,
                                                                                                                            as_str(): "None",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    args: None,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 317,
                                                                                                                end: 329,
                                                                                                                as_str(): "Option::None",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 306,
                                                                                                end: 330,
                                                                                                as_str(): "Result::Ok(Option::None)",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 290,
                                                                                            end: 331,
                                                                                            as_str(): "Option::None => Result::Ok(Option::None),",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 139,
                                                                            end: 339,
                                                                            as_str(): "match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 139,
                                                                    end: 339,
                                                                    as_str(): "match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 139,
                                                            end: 339,
                                                            as_str(): "match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 139,
                                                    end: 339,
                                                    as_str(): "match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 139,
                                            end: 339,
                                            as_str(): "match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 131,
                                    end: 345,
                                    as_str(): "{\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 101,
                                            end: 105,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 101,
                                        end: 105,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 84,
                                end: 345,
                                as_str(): "pub fn transpose(self) -> Result<Option<T>, E> {\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }",
                            },
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 110,
                                        end: 116,
                                        as_str(): "Result",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                31633,
                                            ),
                                            initial_type_id: TypeId(
                                                31633,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0ef5d6490,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                ),
                                                start: 117,
                                                end: 126,
                                                as_str(): "Option<T>",
                                            },
                                        },
                                        TypeArgument {
                                            type_id: TypeId(
                                                31634,
                                            ),
                                            initial_type_id: TypeId(
                                                31634,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0ef5d6490,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                ),
                                                start: 128,
                                                end: 129,
                                                as_str(): "E",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 110,
                                end: 130,
                                as_str(): "Result<Option<T>, E>",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 46,
                        end: 347,
                        as_str(): "impl<T, E> Option<Result<T, E>> {\n    pub fn transpose(self) -> Result<Option<T>, E> {\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 46,
            end: 347,
            as_str(): "impl<T, E> Option<Result<T, E>> {\n    pub fn transpose(self) -> Result<Option<T>, E> {\n      match self {\n          Option::Some(Result::Ok(x)) => Result::Ok(Option::Some(x)),\n          Option::Some(Result::Err(e)) => Result::Err(e),\n          Option::None => Result::Ok(Option::None),\n      }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplSelf(
                ImplSelf {
                    impl_type_parameters: [
                        T: TypeId(31641),
                    ],
                    type_implementing_for: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 357,
                                end: 370,
                                as_str(): "GenericStruct",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [
                                TypeArgument {
                                    type_id: TypeId(
                                        31638,
                                    ),
                                    initial_type_id: TypeId(
                                        31638,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 371,
                                        end: 380,
                                        as_str(): "Option<T>",
                                    },
                                },
                            ],
                        ),
                    },
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 357,
                        end: 381,
                        as_str(): "GenericStruct<Option<T>>",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 395,
                                    end: 404,
                                    as_str(): "transpose",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Public,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: CodeBlock(
                                                    CodeBlock {
                                                        contents: [
                                                            AstNode {
                                                                content: Declaration(
                                                                    VariableDeclaration(
                                                                        VariableDeclaration {
                                                                            name: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "__match_return_var_name_2",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 453,
                                                                                    end: 457,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            type_ascription: Unknown,
                                                                            type_ascription_span: None,
                                                                            body: Expression {
                                                                                kind: Variable(
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 453,
                                                                                            end: 457,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 453,
                                                                                    end: 457,
                                                                                    as_str(): "self",
                                                                                },
                                                                            },
                                                                            is_mutable: false,
                                                                        },
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 447,
                                                                    end: 606,
                                                                    as_str(): "match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }",
                                                                },
                                                            },
                                                            AstNode {
                                                                content: ImplicitReturnExpression(
                                                                    Expression {
                                                                        kind: Match(
                                                                            MatchExpression {
                                                                                value: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: Some(
                                                                                                "__match_return_var_name_2",
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 453,
                                                                                                end: 457,
                                                                                                as_str(): "self",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 453,
                                                                                        end: 457,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                },
                                                                                branches: [
                                                                                    MatchBranch {
                                                                                        scrutinee: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 470,
                                                                                                    end: 483,
                                                                                                    as_str(): "GenericStruct",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 484,
                                                                                                            end: 485,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: Some(
                                                                                                        EnumScrutinee {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 486,
                                                                                                                            end: 492,
                                                                                                                            as_str(): "Option",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 494,
                                                                                                                        end: 498,
                                                                                                                        as_str(): "Some",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: false,
                                                                                                            },
                                                                                                            value: Variable {
                                                                                                                name: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 499,
                                                                                                                        end: 500,
                                                                                                                        as_str(): "y",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 499,
                                                                                                                    end: 500,
                                                                                                                    as_str(): "y",
                                                                                                                },
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 486,
                                                                                                                end: 501,
                                                                                                                as_str(): "Option::Some(y)",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 484,
                                                                                                        end: 501,
                                                                                                        as_str(): "x:Option::Some(y)",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 470,
                                                                                                end: 502,
                                                                                                as_str(): "GenericStruct{x:Option::Some(y)}",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: AmbiguousPathExpression(
                                                                                                AmbiguousPathExpression {
                                                                                                    call_path_binding: TypeBinding {
                                                                                                        inner: CallPath {
                                                                                                            prefixes: [],
                                                                                                            suffix: AmbiguousSuffix {
                                                                                                                before: TypeBinding {
                                                                                                                    inner: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 506,
                                                                                                                            end: 512,
                                                                                                                            as_str(): "Option",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 506,
                                                                                                                        end: 512,
                                                                                                                        as_str(): "Option",
                                                                                                                    },
                                                                                                                },
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 514,
                                                                                                                        end: 518,
                                                                                                                        as_str(): "Some",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 506,
                                                                                                            end: 518,
                                                                                                            as_str(): "Option::Some",
                                                                                                        },
                                                                                                    },
                                                                                                    args: [
                                                                                                        Expression {
                                                                                                            kind: Struct(
                                                                                                                StructExpression {
                                                                                                                    call_path_binding: TypeBinding {
                                                                                                                        inner: CallPath {
                                                                                                                            prefixes: [],
                                                                                                                            suffix: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 519,
                                                                                                                                    end: 532,
                                                                                                                                    as_str(): "GenericStruct",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            is_absolute: false,
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 519,
                                                                                                                            end: 532,
                                                                                                                            as_str(): "GenericStruct",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    fields: [
                                                                                                                        StructExpressionField {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 534,
                                                                                                                                    end: 535,
                                                                                                                                    as_str(): "x",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            value: Expression {
                                                                                                                                kind: Variable(
                                                                                                                                    BaseIdent {
                                                                                                                                        name_override_opt: None,
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 537,
                                                                                                                                            end: 538,
                                                                                                                                            as_str(): "y",
                                                                                                                                        },
                                                                                                                                        is_raw_ident: false,
                                                                                                                                    },
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 537,
                                                                                                                                    end: 538,
                                                                                                                                    as_str(): "y",
                                                                                                                                },
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 534,
                                                                                                                                end: 538,
                                                                                                                                as_str(): "x: y",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 519,
                                                                                                                end: 539,
                                                                                                                as_str(): "GenericStruct{ x: y}",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 506,
                                                                                                end: 540,
                                                                                                as_str(): "Option::Some(GenericStruct{ x: y})",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 470,
                                                                                            end: 541,
                                                                                            as_str(): "GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),",
                                                                                        },
                                                                                    },
                                                                                    MatchBranch {
                                                                                        scrutinee: StructScrutinee {
                                                                                            struct_name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 552,
                                                                                                    end: 565,
                                                                                                    as_str(): "GenericStruct",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            fields: [
                                                                                                Field {
                                                                                                    field: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 566,
                                                                                                            end: 567,
                                                                                                            as_str(): "x",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    scrutinee: Some(
                                                                                                        EnumScrutinee {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 568,
                                                                                                                            end: 574,
                                                                                                                            as_str(): "Option",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                ],
                                                                                                                suffix: BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 576,
                                                                                                                        end: 580,
                                                                                                                        as_str(): "None",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                                is_absolute: false,
                                                                                                            },
                                                                                                            value: CatchAll {
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 568,
                                                                                                                    end: 580,
                                                                                                                    as_str(): "Option::None",
                                                                                                                },
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 568,
                                                                                                                end: 580,
                                                                                                                as_str(): "Option::None",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 566,
                                                                                                        end: 580,
                                                                                                        as_str(): "x:Option::None",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 552,
                                                                                                end: 581,
                                                                                                as_str(): "GenericStruct{x:Option::None}",
                                                                                            },
                                                                                        },
                                                                                        result: Expression {
                                                                                            kind: DelineatedPath(
                                                                                                DelineatedPathExpression {
                                                                                                    call_path_binding: TypeBinding {
                                                                                                        inner: CallPath {
                                                                                                            prefixes: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 585,
                                                                                                                        end: 591,
                                                                                                                        as_str(): "Option",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            suffix: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 593,
                                                                                                                    end: 597,
                                                                                                                    as_str(): "None",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            is_absolute: false,
                                                                                                        },
                                                                                                        type_arguments: [],
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 593,
                                                                                                            end: 597,
                                                                                                            as_str(): "None",
                                                                                                        },
                                                                                                    },
                                                                                                    args: None,
                                                                                                },
                                                                                            ),
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 585,
                                                                                                end: 597,
                                                                                                as_str(): "Option::None",
                                                                                            },
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 552,
                                                                                            end: 598,
                                                                                            as_str(): "GenericStruct{x:Option::None} => Option::None,",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 447,
                                                                            end: 606,
                                                                            as_str(): "match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }",
                                                                        },
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 447,
                                                                    end: 606,
                                                                    as_str(): "match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }",
                                                                },
                                                            },
                                                        ],
                                                        whole_block_span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 447,
                                                            end: 606,
                                                            as_str(): "match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 447,
                                                    end: 606,
                                                    as_str(): "match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 447,
                                            end: 606,
                                            as_str(): "match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 439,
                                    end: 612,
                                    as_str(): "{\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 405,
                                            end: 409,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007fe0fc01dd50,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 405,
                                        end: 409,
                                        as_str(): "self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 388,
                                end: 612,
                                as_str(): "pub fn transpose(self) -> Option<GenericStruct<T>> {\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }",
                            },
                            return_type: Custom {
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0ef5d6490,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                        ),
                                        start: 414,
                                        end: 420,
                                        as_str(): "Option",
                                    },
                                    is_raw_ident: false,
                                },
                                type_arguments: Some(
                                    [
                                        TypeArgument {
                                            type_id: TypeId(
                                                31640,
                                            ),
                                            initial_type_id: TypeId(
                                                31640,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0ef5d6490,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                ),
                                                start: 421,
                                                end: 437,
                                                as_str(): "GenericStruct<T>",
                                            },
                                        },
                                    ],
                                ),
                            },
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007fe0ef5d6490,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                ),
                                start: 414,
                                end: 438,
                                as_str(): "Option<GenericStruct<T>>",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 349,
                        end: 614,
                        as_str(): "impl<T> GenericStruct<Option<T>> {\n    pub fn transpose(self) -> Option<GenericStruct<T>> {\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 349,
            end: 614,
            as_str(): "impl<T> GenericStruct<Option<T>> {\n    pub fn transpose(self) -> Option<GenericStruct<T>> {\n      match self {\n          GenericStruct{x:Option::Some(y)} => Option::Some(GenericStruct{ x: y}),\n          GenericStruct{x:Option::None} => Option::None,\n      }\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            FunctionDeclaration(
                FunctionDeclaration {
                    purity: Pure,
                    attributes: {},
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe0ef5d6490,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                            ),
                            start: 619,
                            end: 623,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 644,
                                                    end: 645,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 647,
                                                        end: 653,
                                                        as_str(): "Option",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                31642,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                31642,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 654,
                                                                end: 669,
                                                                as_str(): "Result<u64, u8>",
                                                            },
                                                        },
                                                    ],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 647,
                                                    end: 670,
                                                    as_str(): "Option<Result<u64, u8>>",
                                                },
                                            ),
                                            body: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 673,
                                                                                end: 679,
                                                                                as_str(): "Option",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 673,
                                                                            end: 679,
                                                                            as_str(): "Option",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 681,
                                                                            end: 685,
                                                                            as_str(): "Some",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 673,
                                                                end: 685,
                                                                as_str(): "Option::Some",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: AmbiguousPathExpression(
                                                                    AmbiguousPathExpression {
                                                                        call_path_binding: TypeBinding {
                                                                            inner: CallPath {
                                                                                prefixes: [],
                                                                                suffix: AmbiguousSuffix {
                                                                                    before: TypeBinding {
                                                                                        inner: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 686,
                                                                                                end: 692,
                                                                                                as_str(): "Result",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_arguments: [],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 686,
                                                                                            end: 692,
                                                                                            as_str(): "Result",
                                                                                        },
                                                                                    },
                                                                                    suffix: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 694,
                                                                                            end: 696,
                                                                                            as_str(): "Ok",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            type_arguments: [],
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                ),
                                                                                start: 686,
                                                                                end: 696,
                                                                                as_str(): "Result::Ok",
                                                                            },
                                                                        },
                                                                        args: [
                                                                            Expression {
                                                                                kind: Literal(
                                                                                    Numeric(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 697,
                                                                                    end: 698,
                                                                                    as_str(): "5",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 686,
                                                                    end: 699,
                                                                    as_str(): "Result::Ok(5)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 673,
                                                    end: 700,
                                                    as_str(): "Option::Some(Result::Ok(5))",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 640,
                                    end: 701,
                                    as_str(): "let y: Option<Result<u64, u8>> = Option::Some(Result::Ok(5));",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 706,
                                                                end: 712,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 706,
                                                        end: 712,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 745,
                                                                                        end: 747,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 745,
                                                                                        end: 747,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 745,
                                                                                    end: 747,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 745,
                                                                        end: 747,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: MethodApplication(
                                                                            MethodApplicationExpression {
                                                                                method_name_binding: TypeBinding {
                                                                                    inner: FromModule {
                                                                                        method_name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 736,
                                                                                                end: 742,
                                                                                                as_str(): "unwrap",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 736,
                                                                                        end: 742,
                                                                                        as_str(): "unwrap",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: MethodApplication(
                                                                                            MethodApplicationExpression {
                                                                                                method_name_binding: TypeBinding {
                                                                                                    inner: FromModule {
                                                                                                        method_name: BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                ),
                                                                                                                start: 727,
                                                                                                                end: 733,
                                                                                                                as_str(): "unwrap",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    },
                                                                                                    type_arguments: [],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 727,
                                                                                                        end: 733,
                                                                                                        as_str(): "unwrap",
                                                                                                    },
                                                                                                },
                                                                                                contract_call_params: [],
                                                                                                arguments: [
                                                                                                    Expression {
                                                                                                        kind: MethodApplication(
                                                                                                            MethodApplicationExpression {
                                                                                                                method_name_binding: TypeBinding {
                                                                                                                    inner: FromModule {
                                                                                                                        method_name: BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 715,
                                                                                                                                end: 724,
                                                                                                                                as_str(): "transpose",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    },
                                                                                                                    type_arguments: [],
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 715,
                                                                                                                        end: 724,
                                                                                                                        as_str(): "transpose",
                                                                                                                    },
                                                                                                                },
                                                                                                                contract_call_params: [],
                                                                                                                arguments: [
                                                                                                                    Expression {
                                                                                                                        kind: Variable(
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 713,
                                                                                                                                    end: 714,
                                                                                                                                    as_str(): "y",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 713,
                                                                                                                            end: 714,
                                                                                                                            as_str(): "y",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ],
                                                                                                            },
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 713,
                                                                                                            end: 726,
                                                                                                            as_str(): "y.transpose()",
                                                                                                        },
                                                                                                    },
                                                                                                ],
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                            ),
                                                                                            start: 713,
                                                                                            end: 735,
                                                                                            as_str(): "y.transpose().unwrap()",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 713,
                                                                            end: 744,
                                                                            as_str(): "y.transpose().unwrap().unwrap()",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 748,
                                                                            end: 749,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 713,
                                                            end: 749,
                                                            as_str(): "y.transpose().unwrap().unwrap() == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 706,
                                            end: 750,
                                            as_str(): "assert(y.transpose().unwrap().unwrap() == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 706,
                                    end: 750,
                                    as_str(): "assert(y.transpose().unwrap().unwrap() == 5)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 761,
                                                    end: 762,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Custom {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 764,
                                                        end: 777,
                                                        as_str(): "GenericStruct",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: Some(
                                                    [
                                                        TypeArgument {
                                                            type_id: TypeId(
                                                                31643,
                                                            ),
                                                            initial_type_id: TypeId(
                                                                31643,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 778,
                                                                end: 789,
                                                                as_str(): "Option<u64>",
                                                            },
                                                        },
                                                    ],
                                                ),
                                            },
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 764,
                                                    end: 790,
                                                    as_str(): "GenericStruct<Option<u64>>",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Struct(
                                                    StructExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 793,
                                                                        end: 806,
                                                                        as_str(): "GenericStruct",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 793,
                                                                end: 806,
                                                                as_str(): "GenericStruct",
                                                            },
                                                        },
                                                        fields: [
                                                            StructExpressionField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 808,
                                                                        end: 809,
                                                                        as_str(): "x",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                value: Expression {
                                                                    kind: AmbiguousPathExpression(
                                                                        AmbiguousPathExpression {
                                                                            call_path_binding: TypeBinding {
                                                                                inner: CallPath {
                                                                                    prefixes: [],
                                                                                    suffix: AmbiguousSuffix {
                                                                                        before: TypeBinding {
                                                                                            inner: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 811,
                                                                                                    end: 817,
                                                                                                    as_str(): "Option",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 811,
                                                                                                end: 817,
                                                                                                as_str(): "Option",
                                                                                            },
                                                                                        },
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                ),
                                                                                                start: 819,
                                                                                                end: 823,
                                                                                                as_str(): "Some",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    },
                                                                                    is_absolute: false,
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 811,
                                                                                    end: 823,
                                                                                    as_str(): "Option::Some",
                                                                                },
                                                                            },
                                                                            args: [
                                                                                Expression {
                                                                                    kind: Literal(
                                                                                        Numeric(
                                                                                            5,
                                                                                        ),
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 824,
                                                                                        end: 825,
                                                                                        as_str(): "5",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 811,
                                                                        end: 826,
                                                                        as_str(): "Option::Some(5)",
                                                                    },
                                                                },
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                    ),
                                                                    start: 808,
                                                                    end: 826,
                                                                    as_str(): "x: Option::Some(5)",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0ef5d6490,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                    ),
                                                    start: 793,
                                                    end: 827,
                                                    as_str(): "GenericStruct{ x: Option::Some(5)}",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 757,
                                    end: 828,
                                    as_str(): "let y: GenericStruct<Option<u64>> = GenericStruct{ x: Option::Some(5)};",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: FunctionApplication(
                                            FunctionApplicationExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                ),
                                                                start: 833,
                                                                end: 839,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0ef5d6490,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                        ),
                                                        start: 833,
                                                        end: 839,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: MethodApplication(
                                                            MethodApplicationExpression {
                                                                method_name_binding: TypeBinding {
                                                                    inner: FromTrait {
                                                                        call_path: CallPath {
                                                                            prefixes: [
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "core",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 865,
                                                                                        end: 867,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 865,
                                                                                        end: 867,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "eq",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                    ),
                                                                                    start: 865,
                                                                                    end: 867,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                        ),
                                                                        start: 865,
                                                                        end: 867,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: MethodApplication(
                                                                                        MethodApplicationExpression {
                                                                                            method_name_binding: TypeBinding {
                                                                                                inner: FromModule {
                                                                                                    method_name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                            ),
                                                                                                            start: 854,
                                                                                                            end: 860,
                                                                                                            as_str(): "unwrap",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                    ),
                                                                                                    start: 854,
                                                                                                    end: 860,
                                                                                                    as_str(): "unwrap",
                                                                                                },
                                                                                            },
                                                                                            contract_call_params: [],
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: MethodApplication(
                                                                                                        MethodApplicationExpression {
                                                                                                            method_name_binding: TypeBinding {
                                                                                                                inner: FromModule {
                                                                                                                    method_name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 842,
                                                                                                                            end: 851,
                                                                                                                            as_str(): "transpose",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0ef5d6490,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 842,
                                                                                                                    end: 851,
                                                                                                                    as_str(): "transpose",
                                                                                                                },
                                                                                                            },
                                                                                                            contract_call_params: [],
                                                                                                            arguments: [
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0ef5d6490,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 840,
                                                                                                                                end: 841,
                                                                                                                                as_str(): "y",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 840,
                                                                                                                        end: 841,
                                                                                                                        as_str(): "y",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                                        ),
                                                                                                        start: 840,
                                                                                                        end: 853,
                                                                                                        as_str(): "y.transpose()",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 840,
                                                                                        end: 862,
                                                                                        as_str(): "y.transpose().unwrap()",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0ef5d6490,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                                        ),
                                                                                        start: 863,
                                                                                        end: 864,
                                                                                        as_str(): "x",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 840,
                                                                            end: 864,
                                                                            as_str(): "y.transpose().unwrap().x",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0ef5d6490,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                                            ),
                                                                            start: 868,
                                                                            end: 869,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0ef5d6490,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                                            ),
                                                            start: 840,
                                                            end: 869,
                                                            as_str(): "y.transpose().unwrap().x == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 833,
                                            end: 870,
                                            as_str(): "assert(y.transpose().unwrap().x == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 833,
                                    end: 870,
                                    as_str(): "assert(y.transpose().unwrap().x == 5)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0ef5d6490,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                            ),
                                            start: 877,
                                            end: 881,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0ef5d6490,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                                    ),
                                    start: 877,
                                    end: 881,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe0ef5d6490,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                            ),
                            start: 634,
                            end: 883,
                            as_str(): "{\n    let y: Option<Result<u64, u8>> = Option::Some(Result::Ok(5));\n    assert(y.transpose().unwrap().unwrap() == 5);\n\n    let y: GenericStruct<Option<u64>> = GenericStruct{ x: Option::Some(5)};\n    assert(y.transpose().unwrap().x == 5);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 616,
                        end: 883,
                        as_str(): "fn main() -> bool {\n    let y: Option<Result<u64, u8>> = Option::Some(Result::Ok(5));\n    assert(y.transpose().unwrap().unwrap() == 5);\n\n    let y: GenericStruct<Option<u64>> = GenericStruct{ x: Option::Some(5)};\n    assert(y.transpose().unwrap().x == 5);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe0ef5d6490,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
                        ),
                        start: 629,
                        end: 633,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe0ef5d6490,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR8pgoPQ/generic_transpose/src/main.sw",
            ),
            start: 616,
            end: 883,
            as_str(): "fn main() -> bool {\n    let y: Option<Result<u64, u8>> = Option::Some(Result::Ok(5));\n    assert(y.transpose().unwrap().unwrap() == 5);\n\n    let y: GenericStruct<Option<u64>> = GenericStruct{ x: Option::Some(5)};\n    assert(y.transpose().unwrap().x == 5);\n\n    true\n}",
        },
    },
]
