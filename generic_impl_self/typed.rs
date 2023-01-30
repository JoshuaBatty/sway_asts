



TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 75,
            end: 79,
            as_str(): "Data",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 87,
                    end: 92,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31678,
            ),
            initial_type_id: TypeId(
                31679,
            ),
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 87,
                end: 95,
                as_str(): "value: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 94,
                end: 95,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(31678),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 68,
        end: 97,
        as_str(): "struct Data<T> {\n  value: T\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13317,
        Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 99,
            end: 227,
            as_str(): "impl<T> Data<T> {\n  fn new(v: T) -> Self {\n    Data {\n      value: v\n    }\n  }\n\n  fn get_value(self) -> T {\n    self.value\n  }\n}",
        },
    ),
)
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 236,
            end: 250,
            as_str(): "DoubleIdentity",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 261,
                    end: 266,
                    as_str(): "first",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31750,
            ),
            initial_type_id: TypeId(
                31752,
            ),
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 261,
                end: 269,
                as_str(): "first: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 268,
                end: 269,
                as_str(): "T",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 273,
                    end: 279,
                    as_str(): "second",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31751,
            ),
            initial_type_id: TypeId(
                31753,
            ),
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 273,
                end: 282,
                as_str(): "second: F",
            },
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 281,
                end: 282,
                as_str(): "F",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 286,
                    end: 291,
                    as_str(): "third",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 286,
                end: 296,
                as_str(): "third: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 293,
                end: 296,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(31750),
        F: TypeId(31751),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 229,
        end: 298,
        as_str(): "struct DoubleIdentity<T, F> {\n  first: T,\n  second: F,\n  third: u64\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13323,
        Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 300,
            end: 670,
            as_str(): "impl<T, F> DoubleIdentity<T, F> {\n  fn new(x: T, y: F) -> DoubleIdentity<T, F> {\n    DoubleIdentity {\n      first: x,\n      second: y,\n      third: 10u64,\n    }\n  }\n\n  fn get_first(self) -> T {\n    let x: T = self.first;\n    x\n  }\n\n  fn get_second(self) -> F {\n    let y: F = self.second;\n    y\n  }\n\n  fn get_third(self) -> u64 {\n    let z: u64 = self.third;\n    z\n  }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        13330,
        Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 672,
            end: 759,
            as_str(): "impl DoubleIdentity<u8, u8> {\n  fn add(self) -> u8 {\n    self.first + self.second\n  }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 764,
            end: 780,
            as_str(): "double_identity2",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 827,
                                            end: 841,
                                            as_str(): "DoubleIdentity",
                                        },
                                        is_raw_ident: false,
                                    },
                                ],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 851,
                                        end: 854,
                                        as_str(): "new",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 343,
                                            end: 344,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 787,
                                                    end: 788,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 855,
                                                end: 856,
                                                as_str(): "x",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31918,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 855,
                                            end: 856,
                                            as_str(): "x",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 349,
                                            end: 350,
                                            as_str(): "y",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 793,
                                                    end: 794,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 858,
                                                end: 859,
                                                as_str(): "y",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            31919,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 858,
                                            end: 859,
                                            as_str(): "y",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13343,
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 336,
                                    end: 464,
                                    as_str(): "fn new(x: T, y: F) -> DoubleIdentity<T, F> {\n    DoubleIdentity {\n      first: x,\n      second: y,\n      third: 10u64,\n    }\n  }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 827,
                                        end: 854,
                                        as_str(): "DoubleIdentity::<T, F>::new",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31762,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f59921e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                            ),
                            start: 827,
                            end: 860,
                            as_str(): "DoubleIdentity::<T, F>::new(x, y)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 827,
                    end: 860,
                    as_str(): "DoubleIdentity::<T, F>::new(x, y)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 787,
                    end: 788,
                    as_str(): "x",
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
            type_id: TypeId(
                31918,
            ),
            initial_type_id: TypeId(
                31920,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 790,
                end: 791,
                as_str(): "T",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 793,
                    end: 794,
                    as_str(): "y",
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
            type_id: TypeId(
                31919,
            ),
            initial_type_id: TypeId(
                31921,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 796,
                end: 797,
                as_str(): "F",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 761,
        end: 862,
        as_str(): "fn double_identity2<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity::<T, F>::new(x, y)\n}",
    },
    attributes: {},
    return_type: TypeId(
        31923,
    ),
    initial_return_type: TypeId(
        31922,
    ),
    type_parameters: [
        T: TypeId(31918),
        F: TypeId(31919),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 802,
        end: 822,
        as_str(): "DoubleIdentity<T, F>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 867,
            end: 882,
            as_str(): "double_identity",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 933,
                                    end: 938,
                                    as_str(): "inner",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 889,
                                            end: 890,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 944,
                                        end: 945,
                                        as_str(): "x",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    32001,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 944,
                                    end: 945,
                                    as_str(): "x",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                32001,
                            ),
                            type_ascription: TypeId(
                                32001,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 940,
                                    end: 941,
                                    as_str(): "T",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 929,
                    end: 946,
                    as_str(): "let inner: T = x;",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: StructExpression {
                            struct_name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 250,
                                    as_str(): "DoubleIdentity",
                                },
                                is_raw_ident: false,
                            },
                            fields: [
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 970,
                                            end: 975,
                                            as_str(): "first",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 933,
                                                    end: 938,
                                                    as_str(): "inner",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 977,
                                                end: 982,
                                                as_str(): "inner",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            32001,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 977,
                                            end: 982,
                                            as_str(): "inner",
                                        },
                                    },
                                },
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 988,
                                            end: 994,
                                            as_str(): "second",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 895,
                                                    end: 896,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 996,
                                                end: 997,
                                                as_str(): "y",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            32002,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 996,
                                            end: 997,
                                            as_str(): "y",
                                        },
                                    },
                                },
                                TyStructExpressionField {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1003,
                                            end: 1008,
                                            as_str(): "third",
                                        },
                                        is_raw_ident: false,
                                    },
                                    value: TyExpression {
                                        expression: Literal(
                                            U64(
                                                20,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1010,
                                            end: 1015,
                                            as_str(): "20u64",
                                        },
                                    },
                                },
                            ],
                            span: Span {
                                src (ptr): 0x00007fe0f59921e0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                ),
                                start: 949,
                                end: 963,
                                as_str(): "DoubleIdentity",
                            },
                        },
                        return_type: TypeId(
                            32023,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f59921e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                            ),
                            start: 949,
                            end: 1020,
                            as_str(): "DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 949,
                    end: 1020,
                    as_str(): "DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 889,
                    end: 890,
                    as_str(): "x",
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
            type_id: TypeId(
                32001,
            ),
            initial_type_id: TypeId(
                32003,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 892,
                end: 893,
                as_str(): "T",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 895,
                    end: 896,
                    as_str(): "y",
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
            type_id: TypeId(
                32002,
            ),
            initial_type_id: TypeId(
                32004,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 898,
                end: 899,
                as_str(): "F",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 864,
        end: 1022,
        as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  let inner: T = x;\n  DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }\n}",
    },
    attributes: {},
    return_type: TypeId(
        32006,
    ),
    initial_return_type: TypeId(
        32005,
    ),
    type_parameters: [
        T: TypeId(32001),
        F: TypeId(32002),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 904,
        end: 924,
        as_str(): "DoubleIdentity<T, F>",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1027,
            end: 1032,
            as_str(): "crazy",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 1064,
                                    end: 1067,
                                    as_str(): "foo",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 236,
                                            end: 250,
                                            as_str(): "DoubleIdentity",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1091,
                                                    end: 1096,
                                                    as_str(): "first",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1039,
                                                            end: 1040,
                                                            as_str(): "x",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1098,
                                                        end: 1099,
                                                        as_str(): "x",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    32052,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1098,
                                                    end: 1099,
                                                    as_str(): "x",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1105,
                                                    end: 1111,
                                                    as_str(): "second",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1045,
                                                            end: 1046,
                                                            as_str(): "y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1113,
                                                        end: 1114,
                                                        as_str(): "y",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    32053,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1113,
                                                    end: 1114,
                                                    as_str(): "y",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1120,
                                                    end: 1125,
                                                    as_str(): "third",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        30,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1127,
                                                    end: 1132,
                                                    as_str(): "30u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 1070,
                                        end: 1084,
                                        as_str(): "DoubleIdentity",
                                    },
                                },
                                return_type: TypeId(
                                    32061,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 1070,
                                    end: 1137,
                                    as_str(): "DoubleIdentity {\n    first: x,\n    second: y,\n    third: 30u64,\n  }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                32061,
                            ),
                            type_ascription: TypeId(
                                32057,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1060,
                    end: 1138,
                    as_str(): "let foo = DoubleIdentity {\n    first: x,\n    second: y,\n    third: 30u64,\n  };",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 1145,
                                        end: 1155,
                                        as_str(): "get_second",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 548,
                                            end: 552,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1064,
                                                    end: 1067,
                                                    as_str(): "foo",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1141,
                                                end: 1144,
                                                as_str(): "foo",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            32061,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1141,
                                            end: 1144,
                                            as_str(): "foo",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13374,
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 534,
                                    end: 598,
                                    as_str(): "fn get_second(self) -> F {\n    let y: F = self.second;\n    y\n  }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 1145,
                                        end: 1155,
                                        as_str(): "get_second",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31755,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f59921e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                            ),
                            start: 1141,
                            end: 1157,
                            as_str(): "foo.get_second()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1141,
                    end: 1157,
                    as_str(): "foo.get_second()",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1039,
                    end: 1040,
                    as_str(): "x",
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
            type_id: TypeId(
                32052,
            ),
            initial_type_id: TypeId(
                32054,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1042,
                end: 1043,
                as_str(): "T",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1045,
                    end: 1046,
                    as_str(): "y",
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
            type_id: TypeId(
                32053,
            ),
            initial_type_id: TypeId(
                32055,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1048,
                end: 1049,
                as_str(): "F",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1024,
        end: 1159,
        as_str(): "fn crazy<T, F>(x: T, y: F) -> F {\n  let foo = DoubleIdentity {\n    first: x,\n    second: y,\n    third: 30u64,\n  };\n  foo.get_second()\n}",
    },
    attributes: {},
    return_type: TypeId(
        32053,
    ),
    initial_return_type: TypeId(
        32056,
    ),
    type_parameters: [
        T: TypeId(32052),
        F: TypeId(32053),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1054,
        end: 1055,
        as_str(): "F",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1166,
            end: 1174,
            as_str(): "MyResult",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(32078),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1182,
                    end: 1184,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                32078,
            ),
            initial_type_id: TypeId(
                32079,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1186,
                end: 1187,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1182,
                end: 1187,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1191,
                    end: 1194,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                50,
            ),
            initial_type_id: TypeId(
                50,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1196,
                end: 1198,
                as_str(): "u8",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1191,
                end: 1198,
                as_str(): "Err: u8",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1161,
        end: 1212,
        as_str(): "enum MyResult<T> {\n  Ok: T,\n  Err: u8 // err code\n}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        13381,
        Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1214,
            end: 1361,
            as_str(): "impl<T> MyResult<T> {\n  fn ok(value: T) -> Self {\n    MyResult::Ok::<T>(value)\n  }\n\n  fn err(code: u8) -> Self {\n    MyResult::Err::<T>(code)\n  }\n}",
        },
    ),
)
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1368,
            end: 1376,
            as_str(): "MyOption",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(32153),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1384,
                    end: 1388,
                    as_str(): "Some",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                32153,
            ),
            initial_type_id: TypeId(
                32154,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1390,
                end: 1391,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1384,
                end: 1391,
                as_str(): "Some: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1395,
                    end: 1399,
                    as_str(): "None",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                32156,
            ),
            initial_type_id: TypeId(
                32155,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1401,
                end: 1403,
                as_str(): "()",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0f59921e0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                ),
                start: 1395,
                end: 1403,
                as_str(): "None: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1363,
        end: 1405,
        as_str(): "enum MyOption<T> {\n  Some: T,\n  None: ()\n}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        13413,
        Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1407,
            end: 1712,
            as_str(): "impl<T> MyOption<T> {\n  fn some(value: T) -> Self {\n    MyOption::Some::<T>(value)\n  }\n\n  fn none() -> Self {\n    MyOption::None::<T>\n  }\n\n  fn to_result(self) -> MyResult<T> {\n    if let MyOption::Some(value) = self {\n      MyResult::<T>::ok(value)\n    } else {\n      MyResult::<T>::err(99u8)\n    }\n  }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        13592,
        Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1714,
            end: 1808,
            as_str(): "impl<T, E> Result<T, E>{\n    fn dummy(t: T) -> Result<T, bool> {\n        Result::Ok(t)\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1813,
            end: 1829,
            as_str(): "result_impl_test",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 1842,
                                    end: 1845,
                                    as_str(): "res",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1868,
                                                end: 1874,
                                                as_str(): "as_u64",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd71a500,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/u128.sw",
                                                    ),
                                                    start: 3608,
                                                    end: 3612,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 1848,
                                                                    end: 1852,
                                                                    as_str(): "U128",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 1854,
                                                                end: 1858,
                                                                as_str(): "from",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    contract_call_params: {},
                                                    arguments: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd71a500,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/u128.sw",
                                                                    ),
                                                                    start: 449,
                                                                    end: 459,
                                                                    as_str(): "components",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Tuple {
                                                                    fields: [
                                                                        TyExpression {
                                                                            expression: Literal(
                                                                                U64(
                                                                                    0,
                                                                                ),
                                                                            ),
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f59921e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                ),
                                                                                start: 1860,
                                                                                end: 1861,
                                                                                as_str(): "0",
                                                                            },
                                                                        },
                                                                        TyExpression {
                                                                            expression: Literal(
                                                                                U64(
                                                                                    13,
                                                                                ),
                                                                            ),
                                                                            return_type: TypeId(
                                                                                21,
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f59921e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                ),
                                                                                start: 1863,
                                                                                end: 1865,
                                                                                as_str(): "13",
                                                                            },
                                                                        },
                                                                    ],
                                                                },
                                                                return_type: TypeId(
                                                                    32652,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 1859,
                                                                    end: 1866,
                                                                    as_str(): "(0, 13)",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        13593,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd71a500,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/u128.sw",
                                                            ),
                                                            start: 441,
                                                            end: 579,
                                                            as_str(): "fn from(components: (u64, u64)) -> U128 {\n        U128 {\n            upper: components.0,\n            lower: components.1,\n        }\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 1848,
                                                                end: 1858,
                                                                as_str(): "U128::from",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    24199,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1848,
                                                    end: 1867,
                                                    as_str(): "U128::from((0, 13))",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13594,
                                        Span {
                                            src (ptr): 0x00007fe0fd71a500,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/u128.sw",
                                            ),
                                            start: 3594,
                                            end: 3783,
                                            as_str(): "pub fn as_u64(self) -> Result<u64, U128Error> {\n        match self.upper {\n            0 => Result::Ok(self.lower),\n            _ => Result::Err(U128Error::LossOfPrecision),\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1868,
                                                end: 1874,
                                                as_str(): "as_u64",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    24523,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 1848,
                                    end: 1876,
                                    as_str(): "U128::from((0, 13)).as_u64()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                24523,
                            ),
                            type_ascription: TypeId(
                                32644,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1838,
                    end: 1877,
                    as_str(): "let res = U128::from((0, 13)).as_u64();",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 1882,
                                        end: 1888,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4202a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1889,
                                                            end: 1890,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1889,
                                                            end: 1890,
                                                            as_str(): "!",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "not",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1889,
                                                        end: 1890,
                                                        as_str(): "!",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc19c540,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2713,
                                                            end: 2717,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f59921e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 1911,
                                                                        end: 1917,
                                                                        as_str(): "unwrap",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fce41eb0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                            ),
                                                                            start: 4032,
                                                                            end: 4036,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f59921e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                            ),
                                                                                            start: 1890,
                                                                                            end: 1896,
                                                                                            as_str(): "Result",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                ],
                                                                                suffix: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f59921e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                        ),
                                                                                        start: 1898,
                                                                                        end: 1903,
                                                                                        as_str(): "dummy",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                is_absolute: false,
                                                                            },
                                                                            contract_call_params: {},
                                                                            arguments: [
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f59921e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                            ),
                                                                                            start: 1752,
                                                                                            end: 1753,
                                                                                            as_str(): "t",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: Literal(
                                                                                            Boolean(
                                                                                                false,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            71,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f59921e0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                            ),
                                                                                            start: 1904,
                                                                                            end: 1909,
                                                                                            as_str(): "false",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                13694,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                    ),
                                                                                    start: 1743,
                                                                                    end: 1806,
                                                                                    as_str(): "fn dummy(t: T) -> Result<T, bool> {\n        Result::Ok(t)\n    }",
                                                                                },
                                                                            ),
                                                                            self_state_idx: None,
                                                                            selector: None,
                                                                            type_binding: Some(
                                                                                TypeBinding {
                                                                                    inner: (),
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0f59921e0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                        ),
                                                                                        start: 1890,
                                                                                        end: 1903,
                                                                                        as_str(): "Result::dummy",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            32811,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f59921e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                            ),
                                                                            start: 1890,
                                                                            end: 1910,
                                                                            as_str(): "Result::dummy(false)",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13744,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fce41eb0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                    ),
                                                                    start: 4018,
                                                                    end: 4161,
                                                                    as_str(): "pub fn unwrap(self) -> T {\n        match self {\n            Result::Ok(inner_value) => inner_value,\n            _ => revert(0),\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f59921e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 1911,
                                                                        end: 1917,
                                                                        as_str(): "unwrap",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            32735,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1890,
                                                            end: 1919,
                                                            as_str(): "Result::dummy(false).unwrap()",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13745,
                                                Span {
                                                    src (ptr): 0x00007fe0fc19c540,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2706,
                                                    end: 2760,
                                                    as_str(): "fn not(self) -> Self {\n        __eq(self, false)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1889,
                                                        end: 1890,
                                                        as_str(): "!",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1889,
                                            end: 1919,
                                            as_str(): "!Result::dummy(false).unwrap()",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13746,
                                Span {
                                    src (ptr): 0x00007fe0fc4202a0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 1882,
                                        end: 1888,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32888,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f59921e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                            ),
                            start: 1882,
                            end: 1920,
                            as_str(): "assert(!Result::dummy(false).unwrap())",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1882,
                    end: 1920,
                    as_str(): "assert(!Result::dummy(false).unwrap())",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 1926,
                                        end: 1932,
                                        as_str(): "assert",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc4202a0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                            ),
                                            start: 686,
                                            end: 695,
                                            as_str(): "condition",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1950,
                                                            end: 1952,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1950,
                                                            end: 1952,
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
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1950,
                                                        end: 1952,
                                                        as_str(): "==",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: true,
                                            },
                                            contract_call_params: {},
                                            arguments: [
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc19c540,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3022,
                                                            end: 3026,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f59921e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 1937,
                                                                        end: 1946,
                                                                        as_str(): "unwrap_or",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fce41eb0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                            ),
                                                                            start: 4600,
                                                                            end: 4604,
                                                                            as_str(): "self",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                    ),
                                                                                    start: 1842,
                                                                                    end: 1845,
                                                                                    as_str(): "res",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0f59921e0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                                ),
                                                                                start: 1933,
                                                                                end: 1936,
                                                                                as_str(): "res",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            24523,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f59921e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                            ),
                                                                            start: 1933,
                                                                            end: 1936,
                                                                            as_str(): "res",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fce41eb0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                            ),
                                                                            start: 4606,
                                                                            end: 4613,
                                                                            as_str(): "default",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f59921e0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                            ),
                                                                            start: 1947,
                                                                            end: 1948,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13748,
                                                                Span {
                                                                    src (ptr): 0x00007fe0fce41eb0,
                                                                    path: Some(
                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                    ),
                                                                    start: 4583,
                                                                    end: 4752,
                                                                    as_str(): "pub fn unwrap_or(self, default: T) -> T {\n        match self {\n            Result::Ok(inner_value) => inner_value,\n            Result::Err(_) => default,\n        }\n    }",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0f59921e0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                        ),
                                                                        start: 1937,
                                                                        end: 1946,
                                                                        as_str(): "unwrap_or",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1933,
                                                            end: 1949,
                                                            as_str(): "res.unwrap_or(5)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc19c540,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3028,
                                                            end: 3033,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                13,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 1953,
                                                            end: 1955,
                                                            as_str(): "13",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13749,
                                                Span {
                                                    src (ptr): 0x00007fe0fc19c540,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3016,
                                                    end: 3082,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        __eq(self, other)\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1950,
                                                        end: 1952,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1933,
                                            end: 1955,
                                            as_str(): "res.unwrap_or(5) == 13",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13750,
                                Span {
                                    src (ptr): 0x00007fe0fc4202a0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/assert.sw",
                                    ),
                                    start: 672,
                                    end: 751,
                                    as_str(): "pub fn assert(condition: bool) {\n    if !condition {\n        revert(0);\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 1926,
                                        end: 1932,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32897,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f59921e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                            ),
                            start: 1926,
                            end: 1956,
                            as_str(): "assert(res.unwrap_or(5) == 13)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1926,
                    end: 1956,
                    as_str(): "assert(res.unwrap_or(5) == 13)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1810,
        end: 1959,
        as_str(): "fn result_impl_test() {\n    let res = U128::from((0, 13)).as_u64();\n    assert(!Result::dummy(false).unwrap());\n    assert(res.unwrap_or(5) == 13);\n}",
    },
    attributes: {},
    return_type: TypeId(
        32643,
    ),
    initial_return_type: TypeId(
        32642,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1810,
        end: 1831,
        as_str(): "fn result_impl_test()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0f59921e0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
            ),
            start: 1964,
            end: 1968,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 1986,
                                    end: 1987,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1990,
                                                end: 2005,
                                                as_str(): "double_identity",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 889,
                                                    end: 890,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2006,
                                                    end: 2010,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 895,
                                                    end: 896,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2012,
                                                    end: 2016,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13757,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 864,
                                            end: 1022,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  let inner: T = x;\n  DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1990,
                                                end: 2005,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    32903,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 1990,
                                    end: 2017,
                                    as_str(): "double_identity(true, true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                32903,
                            ),
                            type_ascription: TypeId(
                                32900,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 1982,
                    end: 2018,
                    as_str(): "let a = double_identity(true, true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2025,
                                    end: 2026,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2029,
                                                end: 2044,
                                                as_str(): "double_identity",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 889,
                                                    end: 890,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2045,
                                                    end: 2050,
                                                    as_str(): "10u32",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 895,
                                                    end: 896,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        43,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2052,
                                                    end: 2057,
                                                    as_str(): "43u64",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13793,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 864,
                                            end: 1022,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  let inner: T = x;\n  DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2029,
                                                end: 2044,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    32922,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2029,
                                    end: 2058,
                                    as_str(): "double_identity(10u32, 43u64)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                32922,
                            ),
                            type_ascription: TypeId(
                                32919,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2021,
                    end: 2059,
                    as_str(): "let b = double_identity(10u32, 43u64);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2066,
                                    end: 2067,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2070,
                                                end: 2086,
                                                as_str(): "double_identity2",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 787,
                                                    end: 788,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2087,
                                                    end: 2091,
                                                    as_str(): "10u8",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 793,
                                                    end: 794,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2093,
                                                    end: 2096,
                                                    as_str(): "1u8",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13841,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 761,
                                            end: 862,
                                            as_str(): "fn double_identity2<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  DoubleIdentity::<T, F>::new(x, y)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2070,
                                                end: 2086,
                                                as_str(): "double_identity2",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    32952,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2070,
                                    end: 2097,
                                    as_str(): "double_identity2(10u8, 1u8)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                32952,
                            ),
                            type_ascription: TypeId(
                                32949,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2062,
                    end: 2098,
                    as_str(): "let c = double_identity2(10u8, 1u8);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2105,
                                    end: 2106,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 236,
                                            end: 250,
                                            as_str(): "DoubleIdentity",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2130,
                                                    end: 2135,
                                                    as_str(): "first",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2137,
                                                    end: 2140,
                                                    as_str(): "1u8",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2146,
                                                    end: 2152,
                                                    as_str(): "second",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        2,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2154,
                                                    end: 2157,
                                                    as_str(): "2u8",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2163,
                                                    end: 2168,
                                                    as_str(): "third",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        40,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2170,
                                                    end: 2175,
                                                    as_str(): "40u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2109,
                                        end: 2123,
                                        as_str(): "DoubleIdentity",
                                    },
                                },
                                return_type: TypeId(
                                    32991,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2109,
                                    end: 2180,
                                    as_str(): "DoubleIdentity {\n    first: 1u8,\n    second: 2u8,\n    third: 40u64,\n  }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                32991,
                            ),
                            type_ascription: TypeId(
                                32987,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2101,
                    end: 2181,
                    as_str(): "let d = DoubleIdentity {\n    first: 1u8,\n    second: 2u8,\n    third: 40u64,\n  };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2188,
                                    end: 2189,
                                    as_str(): "e",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2194,
                                                end: 2204,
                                                as_str(): "get_second",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 548,
                                                    end: 552,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 2105,
                                                            end: 2106,
                                                            as_str(): "d",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 2192,
                                                        end: 2193,
                                                        as_str(): "d",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    32991,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2192,
                                                    end: 2193,
                                                    as_str(): "d",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13868,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 534,
                                            end: 598,
                                            as_str(): "fn get_second(self) -> F {\n    let y: F = self.second;\n    y\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2194,
                                                end: 2204,
                                                as_str(): "get_second",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    32951,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2192,
                                    end: 2206,
                                    as_str(): "d.get_second()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33013,
                            ),
                            type_ascription: TypeId(
                                33013,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2184,
                    end: 2207,
                    as_str(): "let e = d.get_second();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2214,
                                    end: 2215,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2246,
                                                end: 2261,
                                                as_str(): "double_identity",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 889,
                                                    end: 890,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2262,
                                                    end: 2266,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 895,
                                                    end: 896,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2268,
                                                    end: 2272,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13950,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 864,
                                            end: 1022,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  let inner: T = x;\n  DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2246,
                                                end: 2261,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33048,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2246,
                                    end: 2273,
                                    as_str(): "double_identity(true, true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33048,
                            ),
                            type_ascription: TypeId(
                                33026,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2217,
                                    end: 2243,
                                    as_str(): "DoubleIdentity<bool, bool>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2210,
                    end: 2274,
                    as_str(): "let f: DoubleIdentity<bool, bool> = double_identity(true, true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2281,
                                    end: 2282,
                                    as_str(): "g",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2311,
                                                end: 2326,
                                                as_str(): "double_identity",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 889,
                                                    end: 890,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2327,
                                                    end: 2332,
                                                    as_str(): "10u32",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 895,
                                                    end: 896,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        43,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2334,
                                                    end: 2339,
                                                    as_str(): "43u64",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14084,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 864,
                                            end: 1022,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  let inner: T = x;\n  DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2311,
                                                end: 2326,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33100,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2311,
                                    end: 2340,
                                    as_str(): "double_identity(10u32, 43u64)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33100,
                            ),
                            type_ascription: TypeId(
                                33073,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2284,
                                    end: 2308,
                                    as_str(): "DoubleIdentity<u32, u64>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2277,
                    end: 2341,
                    as_str(): "let g: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2348,
                                    end: 2349,
                                    as_str(): "h",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2352,
                                                    end: 2366,
                                                    as_str(): "DoubleIdentity",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2381,
                                                end: 2384,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 343,
                                                    end: 344,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        3,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2385,
                                                    end: 2389,
                                                    as_str(): "3u64",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 349,
                                                    end: 350,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2391,
                                                    end: 2396,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14133,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 336,
                                            end: 464,
                                            as_str(): "fn new(x: T, y: F) -> DoubleIdentity<T, F> {\n    DoubleIdentity {\n      first: x,\n      second: y,\n      third: 10u64,\n    }\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2352,
                                                end: 2384,
                                                as_str(): "DoubleIdentity::<u64, bool>::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33154,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2352,
                                    end: 2397,
                                    as_str(): "DoubleIdentity::<u64, bool>::new(3u64, false)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33154,
                            ),
                            type_ascription: TypeId(
                                33146,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2344,
                    end: 2398,
                    as_str(): "let h = DoubleIdentity::<u64, bool>::new(3u64, false);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2405,
                                    end: 2406,
                                    as_str(): "i",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2409,
                                                end: 2414,
                                                as_str(): "crazy",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1039,
                                                    end: 1040,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        7,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2415,
                                                    end: 2418,
                                                    as_str(): "7u8",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1045,
                                                    end: 1046,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2420,
                                                    end: 2424,
                                                    as_str(): "10u8",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14212,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1024,
                                            end: 1159,
                                            as_str(): "fn crazy<T, F>(x: T, y: F) -> F {\n  let foo = DoubleIdentity {\n    first: x,\n    second: y,\n    third: 30u64,\n  };\n  foo.get_second()\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2409,
                                                end: 2414,
                                                as_str(): "crazy",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33202,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2409,
                                    end: 2425,
                                    as_str(): "crazy(7u8, 10u8)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33200,
                            ),
                            type_ascription: TypeId(
                                33200,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2401,
                    end: 2426,
                    as_str(): "let i = crazy(7u8, 10u8);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2433,
                                    end: 2434,
                                    as_str(): "j",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2442,
                                                    end: 2443,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2442,
                                                    end: 2443,
                                                    as_str(): "+",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "add",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2442,
                                                end: 2443,
                                                as_str(): "+",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: true,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc19c540,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 408,
                                                    end: 412,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2437,
                                                    end: 2441,
                                                    as_str(): "10u8",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc19c540,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 414,
                                                    end: 419,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U8(
                                                        11,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    50,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2444,
                                                    end: 2448,
                                                    as_str(): "11u8",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14213,
                                        Span {
                                            src (ptr): 0x00007fe0fc19c540,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 401,
                                            end: 469,
                                            as_str(): "fn add(self, other: Self) -> Self {\n        __add(self, other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2442,
                                                end: 2443,
                                                as_str(): "+",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    50,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2437,
                                    end: 2448,
                                    as_str(): "10u8 + 11u8",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33227,
                            ),
                            type_ascription: TypeId(
                                33227,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2429,
                    end: 2449,
                    as_str(): "let j = 10u8 + 11u8;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2456,
                                    end: 2457,
                                    as_str(): "k",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2462,
                                                end: 2465,
                                                as_str(): "add",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 711,
                                                    end: 715,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 2105,
                                                            end: 2106,
                                                            as_str(): "d",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 2460,
                                                        end: 2461,
                                                        as_str(): "d",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    32991,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2460,
                                                    end: 2461,
                                                    as_str(): "d",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14238,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 704,
                                            end: 757,
                                            as_str(): "fn add(self) -> u8 {\n    self.first + self.second\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2462,
                                                end: 2465,
                                                as_str(): "add",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    50,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2460,
                                    end: 2467,
                                    as_str(): "d.add()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33245,
                            ),
                            type_ascription: TypeId(
                                33245,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2452,
                    end: 2468,
                    as_str(): "let k = d.add();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2475,
                                    end: 2476,
                                    as_str(): "l",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2479,
                                                    end: 2483,
                                                    as_str(): "Data",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2493,
                                                end: 2496,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 126,
                                                    end: 127,
                                                    as_str(): "v",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2497,
                                                    end: 2502,
                                                    as_str(): "false",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14265,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 119,
                                            end: 177,
                                            as_str(): "fn new(v: T) -> Self {\n    Data {\n      value: v\n    }\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2479,
                                                end: 2496,
                                                as_str(): "Data::<bool>::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33270,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2479,
                                    end: 2503,
                                    as_str(): "Data::<bool>::new(false)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33270,
                            ),
                            type_ascription: TypeId(
                                33262,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2471,
                    end: 2504,
                    as_str(): "let l = Data::<bool>::new(false);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2511,
                                    end: 2512,
                                    as_str(): "m",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 236,
                                            end: 250,
                                            as_str(): "DoubleIdentity",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2573,
                                                    end: 2578,
                                                    as_str(): "first",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: StructExpression {
                                                    struct_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 79,
                                                            as_str(): "Data",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    fields: [
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2593,
                                                                    end: 2598,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            value: TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        1,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2600,
                                                                    end: 2603,
                                                                    as_str(): "1u8",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 2580,
                                                        end: 2584,
                                                        as_str(): "Data",
                                                    },
                                                },
                                                return_type: TypeId(
                                                    33391,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2580,
                                                    end: 2609,
                                                    as_str(): "Data {\n      value: 1u8\n    }",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2615,
                                                    end: 2621,
                                                    as_str(): "second",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: StructExpression {
                                                    struct_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 75,
                                                            end: 79,
                                                            as_str(): "Data",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    fields: [
                                                        TyStructExpressionField {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2636,
                                                                    end: 2641,
                                                                    as_str(): "value",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            value: TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        2,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2643,
                                                                    end: 2646,
                                                                    as_str(): "2u8",
                                                                },
                                                            },
                                                        },
                                                    ],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 2623,
                                                        end: 2627,
                                                        as_str(): "Data",
                                                    },
                                                },
                                                return_type: TypeId(
                                                    33402,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2623,
                                                    end: 2652,
                                                    as_str(): "Data {\n      value: 2u8\n    }",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2658,
                                                    end: 2663,
                                                    as_str(): "third",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2665,
                                                    end: 2669,
                                                    as_str(): "1u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2552,
                                        end: 2566,
                                        as_str(): "DoubleIdentity",
                                    },
                                },
                                return_type: TypeId(
                                    33363,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2552,
                                    end: 2673,
                                    as_str(): "DoubleIdentity {\n    first: Data {\n      value: 1u8\n    },\n    second: Data {\n      value: 2u8\n    },\n    third: 1u64\n  }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33363,
                            ),
                            type_ascription: TypeId(
                                33321,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2514,
                                    end: 2549,
                                    as_str(): "DoubleIdentity<Data<u8>, Data<u64>>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2507,
                    end: 2674,
                    as_str(): "let m: DoubleIdentity<Data<u8>, Data<u64>> = DoubleIdentity {\n    first: Data {\n      value: 1u8\n    },\n    second: Data {\n      value: 2u8\n    },\n    third: 1u64\n  };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2681,
                                    end: 2682,
                                    as_str(): "n",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2685,
                                                    end: 2699,
                                                    as_str(): "DoubleIdentity",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2723,
                                                end: 2726,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 343,
                                                    end: 344,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2727,
                                                                    end: 2731,
                                                                    as_str(): "Data",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 2739,
                                                                end: 2742,
                                                                as_str(): "new",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    contract_call_params: {},
                                                    arguments: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 127,
                                                                    as_str(): "v",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        3,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2743,
                                                                    end: 2746,
                                                                    as_str(): "3u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        14482,
                                                        Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 177,
                                                            as_str(): "fn new(v: T) -> Self {\n    Data {\n      value: v\n    }\n  }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 2727,
                                                                end: 2742,
                                                                as_str(): "Data::<u8>::new",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    33288,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2727,
                                                    end: 2747,
                                                    as_str(): "Data::<u8>::new(3u8)",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 349,
                                                    end: 350,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2749,
                                                                    end: 2753,
                                                                    as_str(): "Data",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 2761,
                                                                end: 2764,
                                                                as_str(): "new",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    contract_call_params: {},
                                                    arguments: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 127,
                                                                    as_str(): "v",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        4,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 2765,
                                                                    end: 2768,
                                                                    as_str(): "4u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        14553,
                                                        Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 177,
                                                            as_str(): "fn new(v: T) -> Self {\n    Data {\n      value: v\n    }\n  }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 2749,
                                                                end: 2764,
                                                                as_str(): "Data::<u8>::new",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    33288,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2749,
                                                    end: 2769,
                                                    as_str(): "Data::<u8>::new(4u8)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14758,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 336,
                                            end: 464,
                                            as_str(): "fn new(x: T, y: F) -> DoubleIdentity<T, F> {\n    DoubleIdentity {\n      first: x,\n      second: y,\n      third: 10u64,\n    }\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2685,
                                                end: 2726,
                                                as_str(): "DoubleIdentity::<Data<u8>, Data<u8>>::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33367,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2685,
                                    end: 2770,
                                    as_str(): "DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8))",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33367,
                            ),
                            type_ascription: TypeId(
                                33411,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2677,
                    end: 2771,
                    as_str(): "let n = DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8));",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2778,
                                    end: 2779,
                                    as_str(): "o",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2810,
                                                end: 2825,
                                                as_str(): "double_identity",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 889,
                                                    end: 890,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2826,
                                                    end: 2830,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 895,
                                                    end: 896,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2832,
                                                    end: 2836,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        14976,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 864,
                                            end: 1022,
                                            as_str(): "fn double_identity<T, F>(x: T, y: F) -> DoubleIdentity<T, F> {\n  let inner: T = x;\n  DoubleIdentity {\n    first: inner,\n    second: y,\n    third: 20u64,\n  }\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2810,
                                                end: 2825,
                                                as_str(): "double_identity",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33719,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2810,
                                    end: 2837,
                                    as_str(): "double_identity(true, true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33719,
                            ),
                            type_ascription: TypeId(
                                33689,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2781,
                                    end: 2807,
                                    as_str(): "DoubleIdentity<bool, bool>",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2774,
                    end: 2838,
                    as_str(): "let o: DoubleIdentity<bool, bool> = double_identity(true, true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2845,
                                    end: 2846,
                                    as_str(): "p",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1368,
                                                end: 1376,
                                                as_str(): "MyOption",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(71),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1384,
                                                        end: 1388,
                                                        as_str(): "Some",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    32154,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1390,
                                                    end: 1391,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1384,
                                                    end: 1391,
                                                    as_str(): "Some: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1395,
                                                        end: 1399,
                                                        as_str(): "None",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    32156,
                                                ),
                                                initial_type_id: TypeId(
                                                    32155,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1401,
                                                    end: 1403,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1395,
                                                    end: 1403,
                                                    as_str(): "None: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1363,
                                            end: 1405,
                                            as_str(): "enum MyOption<T> {\n  Some: T,\n  None: ()\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1384,
                                            end: 1388,
                                            as_str(): "Some",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                Boolean(
                                                    false,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                71,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2872,
                                                end: 2877,
                                                as_str(): "false",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2849,
                                        end: 2857,
                                        as_str(): "MyOption",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2859,
                                        end: 2863,
                                        as_str(): "Some",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [
                                            TypeArgument {
                                                type_id: TypeId(
                                                    71,
                                                ),
                                                initial_type_id: TypeId(
                                                    31656,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2866,
                                                    end: 2870,
                                                    as_str(): "bool",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 2849,
                                            end: 2871,
                                            as_str(): "MyOption::Some::<bool>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    33772,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2859,
                                    end: 2863,
                                    as_str(): "Some",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33772,
                            ),
                            type_ascription: TypeId(
                                33751,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2841,
                    end: 2879,
                    as_str(): "let p = MyOption::Some::<bool>(false);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2886,
                                    end: 2887,
                                    as_str(): "q",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1368,
                                                end: 1376,
                                                as_str(): "MyOption",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(33775),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1384,
                                                        end: 1388,
                                                        as_str(): "Some",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    33775,
                                                ),
                                                initial_type_id: TypeId(
                                                    32154,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1390,
                                                    end: 1391,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1384,
                                                    end: 1391,
                                                    as_str(): "Some: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1395,
                                                        end: 1399,
                                                        as_str(): "None",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    32156,
                                                ),
                                                initial_type_id: TypeId(
                                                    32155,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1401,
                                                    end: 1403,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1395,
                                                    end: 1403,
                                                    as_str(): "None: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1363,
                                            end: 1405,
                                            as_str(): "enum MyOption<T> {\n  Some: T,\n  None: ()\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1384,
                                            end: 1388,
                                            as_str(): "Some",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Tuple {
                                                fields: [],
                                            },
                                            return_type: TypeId(
                                                33797,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2911,
                                                end: 2913,
                                                as_str(): "()",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2890,
                                        end: 2898,
                                        as_str(): "MyOption",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2900,
                                        end: 2904,
                                        as_str(): "Some",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [
                                            TypeArgument {
                                                type_id: TypeId(
                                                    33775,
                                                ),
                                                initial_type_id: TypeId(
                                                    31656,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2907,
                                                    end: 2909,
                                                    as_str(): "()",
                                                },
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 2890,
                                            end: 2910,
                                            as_str(): "MyOption::Some::<()>",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    33798,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2900,
                                    end: 2904,
                                    as_str(): "Some",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33798,
                            ),
                            type_ascription: TypeId(
                                33773,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2882,
                    end: 2915,
                    as_str(): "let q = MyOption::Some::<()>(());",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2922,
                                    end: 2923,
                                    as_str(): "r",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2926,
                                                    end: 2934,
                                                    as_str(): "MyOption",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2943,
                                                end: 2947,
                                                as_str(): "some",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1439,
                                                    end: 1444,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2948,
                                                    end: 2952,
                                                    as_str(): "5u32",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        15022,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1431,
                                            end: 1493,
                                            as_str(): "fn some(value: T) -> Self {\n    MyOption::Some::<T>(value)\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2926,
                                                end: 2947,
                                                as_str(): "MyOption::<u32>::some",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33821,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2926,
                                    end: 2953,
                                    as_str(): "MyOption::<u32>::some(5u32)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33821,
                            ),
                            type_ascription: TypeId(
                                33799,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2918,
                    end: 2954,
                    as_str(): "let r = MyOption::<u32>::some(5u32);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2961,
                                    end: 2962,
                                    as_str(): "s",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 1368,
                                                end: 1376,
                                                as_str(): "MyOption",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [
                                            T: TypeId(33861),
                                        ],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1384,
                                                        end: 1388,
                                                        as_str(): "Some",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    33861,
                                                ),
                                                initial_type_id: TypeId(
                                                    32154,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1390,
                                                    end: 1391,
                                                    as_str(): "T",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1384,
                                                    end: 1391,
                                                    as_str(): "Some: T",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0f59921e0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                        ),
                                                        start: 1395,
                                                        end: 1399,
                                                        as_str(): "None",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    32156,
                                                ),
                                                initial_type_id: TypeId(
                                                    32155,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1401,
                                                    end: 1403,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 1395,
                                                    end: 1403,
                                                    as_str(): "None: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1363,
                                            end: 1405,
                                            as_str(): "enum MyOption<T> {\n  Some: T,\n  None: ()\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1384,
                                            end: 1388,
                                            as_str(): "Some",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U8(
                                                    0,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                50,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2980,
                                                end: 2983,
                                                as_str(): "0u8",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2965,
                                        end: 2973,
                                        as_str(): "MyOption",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 2975,
                                        end: 2979,
                                        as_str(): "Some",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 2965,
                                            end: 2979,
                                            as_str(): "MyOption::Some",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    33882,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2975,
                                    end: 2979,
                                    as_str(): "Some",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33882,
                            ),
                            type_ascription: TypeId(
                                33860,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2957,
                    end: 2985,
                    as_str(): "let s = MyOption::Some(0u8);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2992,
                                    end: 2993,
                                    as_str(): "t",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2996,
                                                    end: 3004,
                                                    as_str(): "MyOption",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 3013,
                                                end: 3017,
                                                as_str(): "none",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        15120,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 1497,
                                            end: 1544,
                                            as_str(): "fn none() -> Self {\n    MyOption::None::<T>\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 2996,
                                                end: 3017,
                                                as_str(): "MyOption::<u64>::none",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33902,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 2996,
                                    end: 3019,
                                    as_str(): "MyOption::<u64>::none()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33902,
                            ),
                            type_ascription: TypeId(
                                33883,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 2988,
                    end: 3020,
                    as_str(): "let t = MyOption::<u64>::none();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 3027,
                                    end: 3028,
                                    as_str(): "u",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 3031,
                                                    end: 3045,
                                                    as_str(): "DoubleIdentity",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 3069,
                                                end: 3072,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 343,
                                                    end: 344,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 3073,
                                                                    end: 3077,
                                                                    as_str(): "Data",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 3085,
                                                                end: 3088,
                                                                as_str(): "new",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    contract_call_params: {},
                                                    arguments: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 127,
                                                                    as_str(): "v",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        3,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 3089,
                                                                    end: 3092,
                                                                    as_str(): "3u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        15229,
                                                        Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 177,
                                                            as_str(): "fn new(v: T) -> Self {\n    Data {\n      value: v\n    }\n  }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 3073,
                                                                end: 3088,
                                                                as_str(): "Data::<u8>::new",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    33288,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 3073,
                                                    end: 3093,
                                                    as_str(): "Data::<u8>::new(3u8)",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 349,
                                                    end: 350,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 3095,
                                                                    end: 3099,
                                                                    as_str(): "Data",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 3107,
                                                                end: 3110,
                                                                as_str(): "new",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    contract_call_params: {},
                                                    arguments: [
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 126,
                                                                    end: 127,
                                                                    as_str(): "v",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        4,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f59921e0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                    ),
                                                                    start: 3111,
                                                                    end: 3114,
                                                                    as_str(): "4u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        15326,
                                                        Span {
                                                            src (ptr): 0x00007fe0f59921e0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                            ),
                                                            start: 119,
                                                            end: 177,
                                                            as_str(): "fn new(v: T) -> Self {\n    Data {\n      value: v\n    }\n  }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0f59921e0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                                ),
                                                                start: 3095,
                                                                end: 3110,
                                                                as_str(): "Data::<u8>::new",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    33288,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 3095,
                                                    end: 3115,
                                                    as_str(): "Data::<u8>::new(4u8)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        15599,
                                        Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 336,
                                            end: 464,
                                            as_str(): "fn new(x: T, y: F) -> DoubleIdentity<T, F> {\n    DoubleIdentity {\n      first: x,\n      second: y,\n      third: 10u64,\n    }\n  }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 3031,
                                                end: 3072,
                                                as_str(): "DoubleIdentity::<Data<u8>, Data<u8>>::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    33367,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 3031,
                                    end: 3116,
                                    as_str(): "DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8))",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                33367,
                            ),
                            type_ascription: TypeId(
                                33947,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 3023,
                    end: 3117,
                    as_str(): "let u = DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8));",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 3123,
                                        end: 3139,
                                        as_str(): "result_impl_test",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                15725,
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 1810,
                                    end: 1959,
                                    as_str(): "fn result_impl_test() {\n    let res = U128::from((0, 13)).as_u64();\n    assert(!Result::dummy(false).unwrap());\n    assert(res.unwrap_or(5) == 13);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 3123,
                                        end: 3139,
                                        as_str(): "result_impl_test",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            34291,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f59921e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                            ),
                            start: 3123,
                            end: 3141,
                            as_str(): "result_impl_test()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 3123,
                    end: 3141,
                    as_str(): "result_impl_test()",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: FunctionApplication {
                            call_path: CallPath {
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 3148,
                                        end: 3157,
                                        as_str(): "get_first",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 481,
                                            end: 485,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f59921e0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                    ),
                                                    start: 2025,
                                                    end: 2026,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0f59921e0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                                ),
                                                start: 3146,
                                                end: 3147,
                                                as_str(): "b",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            32922,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0f59921e0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                            ),
                                            start: 3146,
                                            end: 3147,
                                            as_str(): "b",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                15726,
                                Span {
                                    src (ptr): 0x00007fe0f59921e0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                    ),
                                    start: 468,
                                    end: 530,
                                    as_str(): "fn get_first(self) -> T {\n    let x: T = self.first;\n    x\n  }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0f59921e0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                                        ),
                                        start: 3148,
                                        end: 3157,
                                        as_str(): "get_first",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32920,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0f59921e0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                            ),
                            start: 3146,
                            end: 3159,
                            as_str(): "b.get_first()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0f59921e0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
                    ),
                    start: 3146,
                    end: 3159,
                    as_str(): "b.get_first()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1961,
        end: 3161,
        as_str(): "fn main() -> u32 {\n  let a = double_identity(true, true);\n  let b = double_identity(10u32, 43u64);\n  let c = double_identity2(10u8, 1u8);\n  let d = DoubleIdentity {\n    first: 1u8,\n    second: 2u8,\n    third: 40u64,\n  };\n  let e = d.get_second();\n  let f: DoubleIdentity<bool, bool> = double_identity(true, true);\n  let g: DoubleIdentity<u32, u64> = double_identity(10u32, 43u64);\n  let h = DoubleIdentity::<u64, bool>::new(3u64, false);\n  let i = crazy(7u8, 10u8);\n  let j = 10u8 + 11u8;\n  let k = d.add();\n  let l = Data::<bool>::new(false);\n  let m: DoubleIdentity<Data<u8>, Data<u64>> = DoubleIdentity {\n    first: Data {\n      value: 1u8\n    },\n    second: Data {\n      value: 2u8\n    },\n    third: 1u64\n  };\n  let n = DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8));\n  let o: DoubleIdentity<bool, bool> = double_identity(true, true);\n  let p = MyOption::Some::<bool>(false);\n  let q = MyOption::Some::<()>(());\n  let r = MyOption::<u32>::some(5u32);\n  let s = MyOption::Some(0u8);\n  let t = MyOption::<u64>::none();\n  let u = DoubleIdentity::<Data<u8>, Data<u8>>::new(Data::<u8>::new(3u8), Data::<u8>::new(4u8));\n\n    result_impl_test();\n\n  b.get_first()\n}",
    },
    attributes: {},
    return_type: TypeId(
        33,
    ),
    initial_return_type: TypeId(
        33,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0f59921e0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRmfZBIj/generic_impl_self/src/main.sw",
        ),
        start: 1974,
        end: 1977,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

