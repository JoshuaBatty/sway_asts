[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a1d5dbf10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                            ),
                            start: 13,
                            end: 17,
                            as_str(): "core",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007f8a1d5dbf10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                            ),
                            start: 19,
                            end: 22,
                            as_str(): "ops",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Star,
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007f8a1d5dbf10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
            ),
            start: 9,
            end: 26,
            as_str(): "use core::ops::*;",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007f8a1d5dbf10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                ),
                                start: 33,
                                end: 35,
                                as_str(): "Eq",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Tuple(
                        [
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 41,
                                    end: 44,
                                    as_str(): "u64",
                                },
                            },
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 46,
                                    end: 49,
                                    as_str(): "u64",
                                },
                            },
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 51,
                                    end: 54,
                                    as_str(): "u64",
                                },
                            },
                        ],
                    ),
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007f8a1d5dbf10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                        ),
                        start: 40,
                        end: 55,
                        as_str(): "(u64, u64, u64)",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 65,
                                    end: 67,
                                    as_str(): "eq",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: LazyOperator(
                                                    LazyOperatorExpression {
                                                        op: And,
                                                        lhs: Expression {
                                                            kind: LazyOperator(
                                                                LazyOperatorExpression {
                                                                    op: And,
                                                                    lhs: Expression {
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
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 112,
                                                                                                        end: 114,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 112,
                                                                                                        end: 114,
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
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 112,
                                                                                                    end: 114,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 112,
                                                                                        end: 114,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: TupleIndex(
                                                                                            TupleIndexExpression {
                                                                                                prefix: Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                                ),
                                                                                                                start: 105,
                                                                                                                end: 109,
                                                                                                                as_str(): "self",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 105,
                                                                                                        end: 109,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                },
                                                                                                index: 0,
                                                                                                index_span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 110,
                                                                                                    end: 111,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 105,
                                                                                            end: 111,
                                                                                            as_str(): "self.0",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: TupleIndex(
                                                                                            TupleIndexExpression {
                                                                                                prefix: Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                                ),
                                                                                                                start: 115,
                                                                                                                end: 120,
                                                                                                                as_str(): "other",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 115,
                                                                                                        end: 120,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                },
                                                                                                index: 0,
                                                                                                index_span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 121,
                                                                                                    end: 122,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 115,
                                                                                            end: 122,
                                                                                            as_str(): "other.0",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 105,
                                                                            end: 122,
                                                                            as_str(): "self.0 == other.0",
                                                                        },
                                                                    },
                                                                    rhs: Expression {
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
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 133,
                                                                                                        end: 135,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 133,
                                                                                                        end: 135,
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
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 133,
                                                                                                    end: 135,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 133,
                                                                                        end: 135,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: TupleIndex(
                                                                                            TupleIndexExpression {
                                                                                                prefix: Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                                ),
                                                                                                                start: 126,
                                                                                                                end: 130,
                                                                                                                as_str(): "self",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 126,
                                                                                                        end: 130,
                                                                                                        as_str(): "self",
                                                                                                    },
                                                                                                },
                                                                                                index: 1,
                                                                                                index_span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 131,
                                                                                                    end: 132,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 126,
                                                                                            end: 132,
                                                                                            as_str(): "self.1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: TupleIndex(
                                                                                            TupleIndexExpression {
                                                                                                prefix: Expression {
                                                                                                    kind: Variable(
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                                ),
                                                                                                                start: 136,
                                                                                                                end: 141,
                                                                                                                as_str(): "other",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                        ),
                                                                                                        start: 136,
                                                                                                        end: 141,
                                                                                                        as_str(): "other",
                                                                                                    },
                                                                                                },
                                                                                                index: 1,
                                                                                                index_span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 142,
                                                                                                    end: 143,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 136,
                                                                                            end: 143,
                                                                                            as_str(): "other.1",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 126,
                                                                            end: 143,
                                                                            as_str(): "self.1 == other.1",
                                                                        },
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 105,
                                                                end: 143,
                                                                as_str(): "self.0 == other.0 && self.1 == other.1",
                                                            },
                                                        },
                                                        rhs: Expression {
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
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 154,
                                                                                            end: 156,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 154,
                                                                                            end: 156,
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
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 154,
                                                                                        end: 156,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 154,
                                                                            end: 156,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: TupleIndex(
                                                                                TupleIndexExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 147,
                                                                                                    end: 151,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 147,
                                                                                            end: 151,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                    },
                                                                                    index: 2,
                                                                                    index_span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 152,
                                                                                        end: 153,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 147,
                                                                                end: 153,
                                                                                as_str(): "self.2",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: TupleIndex(
                                                                                TupleIndexExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 157,
                                                                                                    end: 162,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 157,
                                                                                            end: 162,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                    },
                                                                                    index: 2,
                                                                                    index_span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 163,
                                                                                        end: 164,
                                                                                        as_str(): "2",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 157,
                                                                                end: 164,
                                                                                as_str(): "other.2",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 147,
                                                                end: 164,
                                                                as_str(): "self.2 == other.2",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 164,
                                                    as_str(): "self.0 == other.0 && self.1 == other.1 && self.2 == other.2",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 105,
                                            end: 164,
                                            as_str(): "self.0 == other.0 && self.1 == other.1 && self.2 == other.2",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 95,
                                    end: 170,
                                    as_str(): "{\n        self.0 == other.0 && self.1 == other.1 && self.2 == other.2\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 68,
                                            end: 72,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007f8a28022a30,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 68,
                                        end: 72,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 74,
                                            end: 79,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007f8a28022a30,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 81,
                                        end: 85,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007f8a1d5dbf10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                ),
                                start: 62,
                                end: 170,
                                as_str(): "fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1 && self.2 == other.2\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007f8a1d5dbf10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                ),
                                start: 90,
                                end: 94,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007f8a1d5dbf10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                        ),
                        start: 28,
                        end: 172,
                        as_str(): "impl Eq for (u64, u64, u64) {\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1 && self.2 == other.2\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a1d5dbf10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
            ),
            start: 28,
            end: 172,
            as_str(): "impl Eq for (u64, u64, u64) {\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1 && self.2 == other.2\n    }\n}",
        },
    },
    AstNode {
        content: Declaration(
            ImplTrait(
                ImplTrait {
                    impl_type_parameters: [],
                    trait_name: CallPath {
                        prefixes: [],
                        suffix: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007f8a1d5dbf10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                ),
                                start: 179,
                                end: 181,
                                as_str(): "Eq",
                            },
                            is_raw_ident: false,
                        },
                        is_absolute: false,
                    },
                    trait_type_arguments: [],
                    type_implementing_for: Tuple(
                        [
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 187,
                                    end: 190,
                                    as_str(): "u64",
                                },
                            },
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 192,
                                    end: 195,
                                    as_str(): "u64",
                                },
                            },
                        ],
                    ),
                    type_implementing_for_span: Span {
                        src (ptr): 0x00007f8a1d5dbf10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                        ),
                        start: 186,
                        end: 196,
                        as_str(): "(u64, u64)",
                    },
                    functions: [
                        FunctionDeclaration {
                            purity: Pure,
                            attributes: {},
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 206,
                                    end: 208,
                                    as_str(): "eq",
                                },
                                is_raw_ident: false,
                            },
                            visibility: Private,
                            body: CodeBlock {
                                contents: [
                                    AstNode {
                                        content: ImplicitReturnExpression(
                                            Expression {
                                                kind: LazyOperator(
                                                    LazyOperatorExpression {
                                                        op: And,
                                                        lhs: Expression {
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
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 253,
                                                                                            end: 255,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 253,
                                                                                            end: 255,
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
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 253,
                                                                                        end: 255,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 253,
                                                                            end: 255,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: TupleIndex(
                                                                                TupleIndexExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 246,
                                                                                                    end: 250,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 246,
                                                                                            end: 250,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                    },
                                                                                    index: 0,
                                                                                    index_span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 251,
                                                                                        end: 252,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 246,
                                                                                end: 252,
                                                                                as_str(): "self.0",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: TupleIndex(
                                                                                TupleIndexExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 256,
                                                                                                    end: 261,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 256,
                                                                                            end: 261,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                    },
                                                                                    index: 0,
                                                                                    index_span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 262,
                                                                                        end: 263,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 256,
                                                                                end: 263,
                                                                                as_str(): "other.0",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 246,
                                                                end: 263,
                                                                as_str(): "self.0 == other.0",
                                                            },
                                                        },
                                                        rhs: Expression {
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
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 274,
                                                                                            end: 276,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 274,
                                                                                            end: 276,
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
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 274,
                                                                                        end: 276,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: true,
                                                                            },
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 274,
                                                                            end: 276,
                                                                            as_str(): "==",
                                                                        },
                                                                    },
                                                                    contract_call_params: [],
                                                                    arguments: [
                                                                        Expression {
                                                                            kind: TupleIndex(
                                                                                TupleIndexExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 267,
                                                                                                    end: 271,
                                                                                                    as_str(): "self",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 267,
                                                                                            end: 271,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                    },
                                                                                    index: 1,
                                                                                    index_span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 272,
                                                                                        end: 273,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 267,
                                                                                end: 273,
                                                                                as_str(): "self.1",
                                                                            },
                                                                        },
                                                                        Expression {
                                                                            kind: TupleIndex(
                                                                                TupleIndexExpression {
                                                                                    prefix: Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                                    ),
                                                                                                    start: 277,
                                                                                                    end: 282,
                                                                                                    as_str(): "other",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                            ),
                                                                                            start: 277,
                                                                                            end: 282,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                    },
                                                                                    index: 1,
                                                                                    index_span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 283,
                                                                                        end: 284,
                                                                                        as_str(): "1",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                ),
                                                                                start: 277,
                                                                                end: 284,
                                                                                as_str(): "other.1",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 267,
                                                                end: 284,
                                                                as_str(): "self.1 == other.1",
                                                            },
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 246,
                                                    end: 284,
                                                    as_str(): "self.0 == other.0 && self.1 == other.1",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 246,
                                            end: 284,
                                            as_str(): "self.0 == other.0 && self.1 == other.1",
                                        },
                                    },
                                ],
                                whole_block_span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 290,
                                    as_str(): "{\n        self.0 == other.0 && self.1 == other.1\n    }",
                                },
                            },
                            parameters: [
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 209,
                                            end: 213,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007f8a28022a30,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 209,
                                        end: 213,
                                        as_str(): "self",
                                    },
                                },
                                FunctionParameter {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 215,
                                            end: 220,
                                            as_str(): "other",
                                        },
                                        is_raw_ident: false,
                                    },
                                    is_reference: false,
                                    is_mutable: false,
                                    mutability_span: Span {
                                        src (ptr): 0x00007f8a28022a30,
                                        path: None,
                                        start: 0,
                                        end: 0,
                                        as_str(): "",
                                    },
                                    type_info: SelfType,
                                    type_span: Span {
                                        src (ptr): 0x00007f8a1d5dbf10,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                        ),
                                        start: 222,
                                        end: 226,
                                        as_str(): "Self",
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007f8a1d5dbf10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                ),
                                start: 203,
                                end: 290,
                                as_str(): "fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1\n    }",
                            },
                            return_type: Boolean,
                            type_parameters: [],
                            return_type_span: Span {
                                src (ptr): 0x00007f8a1d5dbf10,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                ),
                                start: 231,
                                end: 235,
                                as_str(): "bool",
                            },
                        },
                    ],
                    block_span: Span {
                        src (ptr): 0x00007f8a1d5dbf10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                        ),
                        start: 174,
                        end: 292,
                        as_str(): "impl Eq for (u64, u64) {\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1\n    }\n}",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a1d5dbf10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
            ),
            start: 174,
            end: 292,
            as_str(): "impl Eq for (u64, u64) {\n    fn eq(self, other: Self) -> bool {\n        self.0 == other.0 && self.1 == other.1\n    }\n}",
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
                            src (ptr): 0x00007f8a1d5dbf10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                            ),
                            start: 297,
                            end: 301,
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
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 322,
                                                    end: 323,
                                                    as_str(): "t",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            body: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    42,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 327,
                                                                end: 329,
                                                                as_str(): "42",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    43,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 331,
                                                                end: 333,
                                                                as_str(): "43",
                                                            },
                                                        },
                                                    ],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                    ),
                                                    start: 326,
                                                    end: 334,
                                                    as_str(): "(42, 43)",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 318,
                                    end: 335,
                                    as_str(): "let t = (42, 43);",
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
                                                                src (ptr): 0x00007f8a1d5dbf10,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                ),
                                                                start: 340,
                                                                end: 346,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                        ),
                                                        start: 340,
                                                        end: 346,
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
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 349,
                                                                                        end: 351,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                        ),
                                                                                        start: 349,
                                                                                        end: 351,
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
                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                    ),
                                                                                    start: 349,
                                                                                    end: 351,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007f8a1d5dbf10,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                        ),
                                                                        start: 349,
                                                                        end: 351,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                    ),
                                                                                    start: 347,
                                                                                    end: 348,
                                                                                    as_str(): "t",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 347,
                                                                            end: 348,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007f8a1d5dbf10,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                                    ),
                                                                                    start: 352,
                                                                                    end: 353,
                                                                                    as_str(): "t",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                                            ),
                                                                            start: 352,
                                                                            end: 353,
                                                                            as_str(): "t",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007f8a1d5dbf10,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                                            ),
                                                            start: 347,
                                                            end: 353,
                                                            as_str(): "t == t",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 340,
                                            end: 354,
                                            as_str(): "assert(t == t)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 340,
                                    end: 354,
                                    as_str(): "assert(t == t)",
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
                                            src (ptr): 0x00007f8a1d5dbf10,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                            ),
                                            start: 361,
                                            end: 365,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007f8a1d5dbf10,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                                    ),
                                    start: 361,
                                    end: 365,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007f8a1d5dbf10,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                            ),
                            start: 312,
                            end: 367,
                            as_str(): "{\n    let t = (42, 43);\n    assert(t == t);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007f8a1d5dbf10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                        ),
                        start: 294,
                        end: 367,
                        as_str(): "fn main() -> bool {\n    let t = (42, 43);\n    assert(t == t);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007f8a1d5dbf10,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
                        ),
                        start: 307,
                        end: 311,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007f8a1d5dbf10,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR79o0Ye/tuple_trait/src/main.sw",
            ),
            start: 294,
            end: 367,
            as_str(): "fn main() -> bool {\n    let t = (42, 43);\n    assert(t == t);\n\n    true\n}",
        },
    },
]
