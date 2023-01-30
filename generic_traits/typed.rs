




TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 338,
            end: 344,
            as_str(): "Setter",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7365),
    ],
    interface_surface: [
        DeclId(
            570,
            Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 357,
                end: 360,
                as_str(): "set",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 332,
        end: 391,
        as_str(): "trait Setter<T> {\n    fn set(self, new_value: T) -> Self;\n}",
    },
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 400,
            end: 410,
            as_str(): "FooBarData",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 420,
                    end: 425,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7383,
            ),
            initial_type_id: TypeId(
                7384,
            ),
            span: Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 420,
                end: 428,
                as_str(): "value: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 427,
                end: 428,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7383),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 393,
        end: 430,
        as_str(): "struct FooBarData<T> {\n    value: T\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        576,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 432,
            end: 579,
            as_str(): "impl<T> Setter<T> for FooBarData<T> {\n    fn set(self, new_value: T) -> Self {\n        FooBarData {\n            value: new_value,\n        }\n    }\n}",
        },
    ),
)
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 587,
            end: 595,
            as_str(): "Returner",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7423),
    ],
    interface_surface: [
        DeclId(
            577,
            Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 608,
                end: 617,
                as_str(): "return_it",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 581,
        end: 645,
        as_str(): "trait Returner<T> {\n    fn return_it(self, the_value: T) -> T;\n}",
    },
}
ImplTrait(
    DeclId(
        583,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 647,
            end: 759,
            as_str(): "impl<T, F> Returner<T> for FooBarData<F> {\n    fn return_it(self, the_value: T) -> T {\n        the_value\n    }\n}",
        },
    ),
)
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 767,
            end: 772,
            as_str(): "MyAdd",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7483),
    ],
    interface_surface: [
        DeclId(
            584,
            Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 785,
                end: 791,
                as_str(): "my_add",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 761,
        end: 817,
        as_str(): "trait MyAdd<T> {\n    fn my_add(self, a: T, b: T) -> T;\n}",
    },
}
ImplTrait(
    DeclId(
        592,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 937,
            end: 1042,
            as_str(): "impl<T> MyAdd<u64> for FooBarData<T> {\n    fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }\n}",
        },
    ),
)
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1050,
            end: 1055,
            as_str(): "MySub",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(7550),
    ],
    interface_surface: [
        DeclId(
            593,
            Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 1068,
                end: 1074,
                as_str(): "my_sub",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 1044,
        end: 1094,
        as_str(): "trait MySub<T> {\n    fn my_sub(a: T, b: T) -> T;\n}",
    },
}
ImplTrait(
    DeclId(
        606,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1289,
            end: 1457,
            as_str(): "impl<T> MySub<u64> for FooBarData<T> {\n    fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }\n}",
        },
    ),
)
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1466,
            end: 1475,
            as_str(): "OtherData",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 1485,
                    end: 1486,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7625,
            ),
            initial_type_id: TypeId(
                7626,
            ),
            span: Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 1485,
                end: 1489,
                as_str(): "a: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 1488,
                end: 1489,
                as_str(): "T",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 1495,
                    end: 1496,
                    as_str(): "b",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7625,
            ),
            initial_type_id: TypeId(
                7627,
            ),
            span: Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 1495,
                end: 1499,
                as_str(): "b: T",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 1498,
                end: 1499,
                as_str(): "T",
            },
            attributes: {},
        },
    ],
    type_parameters: [
        T: TypeId(7625),
    ],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 1459,
        end: 1502,
        as_str(): "struct OtherData<T> {\n    a: T,\n    b: T,\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        611,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1621,
            end: 1725,
            as_str(): "impl<T> MyAdd<u64> for OtherData<T> {\n    fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        620,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 1919,
            end: 2086,
            as_str(): "impl<T> MySub<u64> for OtherData<T> {\n    fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        639,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2088,
            end: 2221,
            as_str(): "impl MyTriple<u64> for MyPoint<u64> {\n    fn my_triple(self, value: u64) -> u64 {\n        (self.x*3) + (self.y*3) + (value*3)\n    }\n}",
        },
    ),
)
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2230,
            end: 2235,
            as_str(): "MyU64",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2242,
                    end: 2247,
                    as_str(): "inner",
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
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 2242,
                end: 2252,
                as_str(): "inner: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fd90ad70,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                ),
                start: 2249,
                end: 2252,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 2223,
        end: 2254,
        as_str(): "struct MyU64 {\n    inner: u64\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        646,
        Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2256,
            end: 2373,
            as_str(): "impl MyTriple<u64> for MyU64 {\n    fn my_triple(self, value: u64) -> u64 {\n        (self.inner*3) + (value*3)\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fd90ad70,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
            ),
            start: 2378,
            end: 2382,
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
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2402,
                                    end: 2403,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 400,
                                            end: 410,
                                            as_str(): "FooBarData",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2427,
                                                    end: 2432,
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2434,
                                                    end: 2437,
                                                    as_str(): "1u8",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2406,
                                        end: 2416,
                                        as_str(): "FooBarData",
                                    },
                                },
                                return_type: TypeId(
                                    7832,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2406,
                                    end: 2443,
                                    as_str(): "FooBarData {\n        value: 1u8\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7832,
                            ),
                            type_ascription: TypeId(
                                7829,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2398,
                    end: 2444,
                    as_str(): "let a = FooBarData {\n        value: 1u8\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2453,
                                    end: 2454,
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2459,
                                                end: 2462,
                                                as_str(): "set",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
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
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 2402,
                                                            end: 2403,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 2457,
                                                        end: 2458,
                                                        as_str(): "a",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7832,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2457,
                                                    end: 2458,
                                                    as_str(): "a",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 487,
                                                    end: 496,
                                                    as_str(): "new_value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        42,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2463,
                                                    end: 2465,
                                                    as_str(): "42",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        663,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 474,
                                            end: 577,
                                            as_str(): "fn set(self, new_value: T) -> Self {\n        FooBarData {\n            value: new_value,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2459,
                                                end: 2462,
                                                as_str(): "set",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7840,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2457,
                                    end: 2466,
                                    as_str(): "a.set(42)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7840,
                            ),
                            type_ascription: TypeId(
                                7843,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2449,
                    end: 2467,
                    as_str(): "let b = a.set(42);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2476,
                                    end: 2477,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructFieldAccess {
                                    prefix: TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2453,
                                                    end: 2454,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2480,
                                                end: 2481,
                                                as_str(): "b",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7840,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2480,
                                            end: 2481,
                                            as_str(): "b",
                                        },
                                    },
                                    field_to_access: TyStructField {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 420,
                                                end: 425,
                                                as_str(): "value",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_id: TypeId(
                                            7831,
                                        ),
                                        initial_type_id: TypeId(
                                            7384,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 420,
                                            end: 428,
                                            as_str(): "value: T",
                                        },
                                        type_span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 427,
                                            end: 428,
                                            as_str(): "T",
                                        },
                                        attributes: {},
                                    },
                                    field_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2482,
                                        end: 2487,
                                        as_str(): "value",
                                    },
                                    resolved_type_of_parent: TypeId(
                                        7840,
                                    ),
                                },
                                return_type: TypeId(
                                    7831,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2480,
                                    end: 2487,
                                    as_str(): "b.value",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7861,
                            ),
                            type_ascription: TypeId(
                                7861,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2472,
                    end: 2488,
                    as_str(): "let c = b.value;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2497,
                                    end: 2498,
                                    as_str(): "d",
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2503,
                                                end: 2512,
                                                as_str(): "return_it",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 707,
                                                    end: 711,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 2453,
                                                            end: 2454,
                                                            as_str(): "b",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 2501,
                                                        end: 2502,
                                                        as_str(): "b",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7840,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2501,
                                                    end: 2502,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 713,
                                                    end: 722,
                                                    as_str(): "the_value",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2513,
                                                    end: 2517,
                                                    as_str(): "true",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        699,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 694,
                                            end: 757,
                                            as_str(): "fn return_it(self, the_value: T) -> T {\n        the_value\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2503,
                                                end: 2512,
                                                as_str(): "return_it",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7866,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2501,
                                    end: 2518,
                                    as_str(): "b.return_it(true)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7866,
                            ),
                            type_ascription: TypeId(
                                7863,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2493,
                    end: 2519,
                    as_str(): "let d = b.return_it(true);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2528,
                                    end: 2529,
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2534,
                                                end: 2543,
                                                as_str(): "return_it",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 707,
                                                    end: 711,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 2453,
                                                            end: 2454,
                                                            as_str(): "b",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 2532,
                                                        end: 2533,
                                                        as_str(): "b",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7840,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2532,
                                                    end: 2533,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 713,
                                                    end: 722,
                                                    as_str(): "the_value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        9,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2544,
                                                    end: 2548,
                                                    as_str(): "9u64",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        700,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 694,
                                            end: 757,
                                            as_str(): "fn return_it(self, the_value: T) -> T {\n        the_value\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2534,
                                                end: 2543,
                                                as_str(): "return_it",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7870,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2532,
                                    end: 2549,
                                    as_str(): "b.return_it(9u64)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7867,
                            ),
                            type_ascription: TypeId(
                                7867,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2524,
                    end: 2550,
                    as_str(): "let e = b.return_it(9u64);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2559,
                                    end: 2560,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 400,
                                            end: 410,
                                            as_str(): "FooBarData",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2584,
                                                    end: 2589,
                                                    as_str(): "value",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2591,
                                                    end: 2595,
                                                    as_str(): "1u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2563,
                                        end: 2573,
                                        as_str(): "FooBarData",
                                    },
                                },
                                return_type: TypeId(
                                    7875,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2563,
                                    end: 2601,
                                    as_str(): "FooBarData {\n        value: 1u64\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7875,
                            ),
                            type_ascription: TypeId(
                                7872,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2555,
                    end: 2602,
                    as_str(): "let f = FooBarData {\n        value: 1u64\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2611,
                                    end: 2612,
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2617,
                                                end: 2623,
                                                as_str(): "my_add",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 990,
                                                    end: 994,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 2559,
                                                            end: 2560,
                                                            as_str(): "f",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 2615,
                                                        end: 2616,
                                                        as_str(): "f",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7875,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2615,
                                                    end: 2616,
                                                    as_str(): "f",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 996,
                                                    end: 997,
                                                    as_str(): "a",
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
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2626,
                                                                end: 2632,
                                                                as_str(): "my_add",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 990,
                                                                    end: 994,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2402,
                                                                            end: 2403,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2624,
                                                                        end: 2625,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7832,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2624,
                                                                    end: 2625,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 996,
                                                                    end: 997,
                                                                    as_str(): "a",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2633,
                                                                    end: 2636,
                                                                    as_str(): "1u8",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1004,
                                                                    end: 1005,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        2,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2638,
                                                                    end: 2641,
                                                                    as_str(): "2u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        720,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 980,
                                                            end: 1040,
                                                            as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2626,
                                                                end: 2632,
                                                                as_str(): "my_add",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2624,
                                                    end: 2642,
                                                    as_str(): "a.my_add(1u8, 2u8)",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1004,
                                                    end: 1005,
                                                    as_str(): "b",
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
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2646,
                                                                end: 2652,
                                                                as_str(): "my_add",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 990,
                                                                    end: 994,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2402,
                                                                            end: 2403,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2644,
                                                                        end: 2645,
                                                                        as_str(): "a",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7832,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2644,
                                                                    end: 2645,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 996,
                                                                    end: 997,
                                                                    as_str(): "a",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2653,
                                                                    end: 2656,
                                                                    as_str(): "3u8",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1004,
                                                                    end: 1005,
                                                                    as_str(): "b",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2658,
                                                                    end: 2661,
                                                                    as_str(): "4u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        724,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 980,
                                                            end: 1040,
                                                            as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2646,
                                                                end: 2652,
                                                                as_str(): "my_add",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2644,
                                                    end: 2662,
                                                    as_str(): "a.my_add(3u8, 4u8)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        728,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 980,
                                            end: 1040,
                                            as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2617,
                                                end: 2623,
                                                as_str(): "my_add",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2615,
                                    end: 2663,
                                    as_str(): "f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8))",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7886,
                            ),
                            type_ascription: TypeId(
                                7886,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2607,
                    end: 2664,
                    as_str(): "let g = f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8));",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2673,
                                    end: 2674,
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2677,
                                                    end: 2687,
                                                    as_str(): "FooBarData",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2696,
                                                end: 2702,
                                                as_str(): "my_sub",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1342,
                                                    end: 1343,
                                                    as_str(): "a",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2712,
                                                                    end: 2722,
                                                                    as_str(): "FooBarData",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2730,
                                                                end: 2736,
                                                                as_str(): "my_sub",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1342,
                                                                    end: 1343,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        100,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2737,
                                                                    end: 2740,
                                                                    as_str(): "100",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1350,
                                                                    end: 1351,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        10,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2742,
                                                                    end: 2744,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        783,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 1332,
                                                            end: 1455,
                                                            as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2712,
                                                                end: 2736,
                                                                as_str(): "FooBarData::<u8>::my_sub",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2712,
                                                    end: 2745,
                                                    as_str(): "FooBarData::<u8>::my_sub(100, 10)",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1350,
                                                    end: 1351,
                                                    as_str(): "b",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2755,
                                                                    end: 2765,
                                                                    as_str(): "FooBarData",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2773,
                                                                end: 2779,
                                                                as_str(): "my_sub",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1342,
                                                                    end: 1343,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        50,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2780,
                                                                    end: 2782,
                                                                    as_str(): "50",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1350,
                                                                    end: 1351,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        10,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2784,
                                                                    end: 2786,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        854,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 1332,
                                                            end: 1455,
                                                            as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2755,
                                                                end: 2779,
                                                                as_str(): "FooBarData::<u8>::my_sub",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2755,
                                                    end: 2787,
                                                    as_str(): "FooBarData::<u8>::my_sub(50, 10)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        893,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1332,
                                            end: 1455,
                                            as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2677,
                                                end: 2702,
                                                as_str(): "FooBarData::<u64>::my_sub",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2677,
                                    end: 2794,
                                    as_str(): "FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    )",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7899,
                            ),
                            type_ascription: TypeId(
                                7899,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2669,
                    end: 2795,
                    as_str(): "let h = FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2804,
                                    end: 2805,
                                    as_str(): "i",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1466,
                                            end: 1475,
                                            as_str(): "OtherData",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2828,
                                                    end: 2829,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        true,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2831,
                                                    end: 2835,
                                                    as_str(): "true",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2845,
                                                    end: 2846,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    Boolean(
                                                        false,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2848,
                                                    end: 2853,
                                                    as_str(): "false",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2808,
                                        end: 2817,
                                        as_str(): "OtherData",
                                    },
                                },
                                return_type: TypeId(
                                    7976,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2808,
                                    end: 2860,
                                    as_str(): "OtherData {\n        a: true,\n        b: false,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7976,
                            ),
                            type_ascription: TypeId(
                                7973,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2800,
                    end: 2861,
                    as_str(): "let i = OtherData {\n        a: true,\n        b: false,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2870,
                                    end: 2871,
                                    as_str(): "j",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1466,
                                            end: 1475,
                                            as_str(): "OtherData",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2894,
                                                    end: 2895,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2897,
                                                    end: 2902,
                                                    as_str(): "10u32",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2912,
                                                    end: 2913,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U32(
                                                        11,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    33,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2915,
                                                    end: 2920,
                                                    as_str(): "11u32",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 2874,
                                        end: 2883,
                                        as_str(): "OtherData",
                                    },
                                },
                                return_type: TypeId(
                                    7985,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2874,
                                    end: 2927,
                                    as_str(): "OtherData {\n        a: 10u32,\n        b: 11u32,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7985,
                            ),
                            type_ascription: TypeId(
                                7982,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2866,
                    end: 2928,
                    as_str(): "let j = OtherData {\n        a: 10u32,\n        b: 11u32,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2937,
                                    end: 2938,
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2943,
                                                end: 2949,
                                                as_str(): "my_add",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1673,
                                                    end: 1677,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 2870,
                                                            end: 2871,
                                                            as_str(): "j",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 2941,
                                                        end: 2942,
                                                        as_str(): "j",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7985,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2941,
                                                    end: 2942,
                                                    as_str(): "j",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1679,
                                                    end: 1680,
                                                    as_str(): "a",
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
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2952,
                                                                end: 2958,
                                                                as_str(): "my_add",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1673,
                                                                    end: 1677,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2804,
                                                                            end: 2805,
                                                                            as_str(): "i",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2950,
                                                                        end: 2951,
                                                                        as_str(): "i",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7976,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2950,
                                                                    end: 2951,
                                                                    as_str(): "i",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1679,
                                                                    end: 1680,
                                                                    as_str(): "a",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2959,
                                                                    end: 2962,
                                                                    as_str(): "1u8",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1687,
                                                                    end: 1688,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U8(
                                                                        2,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    50,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2964,
                                                                    end: 2967,
                                                                    as_str(): "2u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        925,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 1663,
                                                            end: 1723,
                                                            as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2952,
                                                                end: 2958,
                                                                as_str(): "my_add",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2950,
                                                    end: 2968,
                                                    as_str(): "i.my_add(1u8, 2u8)",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1687,
                                                    end: 1688,
                                                    as_str(): "b",
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
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2972,
                                                                end: 2978,
                                                                as_str(): "my_add",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1673,
                                                                    end: 1677,
                                                                    as_str(): "self",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 2804,
                                                                            end: 2805,
                                                                            as_str(): "i",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 2970,
                                                                        end: 2971,
                                                                        as_str(): "i",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    7976,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2970,
                                                                    end: 2971,
                                                                    as_str(): "i",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1679,
                                                                    end: 1680,
                                                                    as_str(): "a",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2979,
                                                                    end: 2982,
                                                                    as_str(): "3u8",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1687,
                                                                    end: 1688,
                                                                    as_str(): "b",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 2984,
                                                                    end: 2987,
                                                                    as_str(): "4u8",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        929,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 1663,
                                                            end: 1723,
                                                            as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 2972,
                                                                end: 2978,
                                                                as_str(): "my_add",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2970,
                                                    end: 2988,
                                                    as_str(): "i.my_add(3u8, 4u8)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        933,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1663,
                                            end: 1723,
                                            as_str(): "fn my_add(self, a: u64, b: u64) -> u64 {\n        a + b\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 2943,
                                                end: 2949,
                                                as_str(): "my_add",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2941,
                                    end: 2989,
                                    as_str(): "j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8))",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7991,
                            ),
                            type_ascription: TypeId(
                                7991,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2933,
                    end: 2990,
                    as_str(): "let k = j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8));",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 2999,
                                    end: 3000,
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3003,
                                                    end: 3013,
                                                    as_str(): "FooBarData",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3022,
                                                end: 3028,
                                                as_str(): "my_sub",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1342,
                                                    end: 1343,
                                                    as_str(): "a",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3038,
                                                                    end: 3048,
                                                                    as_str(): "FooBarData",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3057,
                                                                end: 3063,
                                                                as_str(): "my_sub",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1342,
                                                                    end: 1343,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        100,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3064,
                                                                    end: 3067,
                                                                    as_str(): "100",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1350,
                                                                    end: 1351,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        10,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3069,
                                                                    end: 3071,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        953,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 1332,
                                                            end: 1455,
                                                            as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3038,
                                                                end: 3063,
                                                                as_str(): "FooBarData::<u32>::my_sub",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3038,
                                                    end: 3072,
                                                    as_str(): "FooBarData::<u32>::my_sub(100, 10)",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 1350,
                                                    end: 1351,
                                                    as_str(): "b",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3082,
                                                                    end: 3092,
                                                                    as_str(): "FooBarData",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ],
                                                        suffix: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3101,
                                                                end: 3107,
                                                                as_str(): "my_sub",
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
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1342,
                                                                    end: 1343,
                                                                    as_str(): "a",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        50,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3108,
                                                                    end: 3110,
                                                                    as_str(): "50",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 1350,
                                                                    end: 1351,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            TyExpression {
                                                                expression: Literal(
                                                                    U64(
                                                                        10,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3112,
                                                                    end: 3114,
                                                                    as_str(): "10",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        989,
                                                        Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 1332,
                                                            end: 1455,
                                                            as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                                        },
                                                    ),
                                                    self_state_idx: None,
                                                    selector: None,
                                                    type_binding: Some(
                                                        TypeBinding {
                                                            inner: (),
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3082,
                                                                end: 3107,
                                                                as_str(): "FooBarData::<u32>::my_sub",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3082,
                                                    end: 3115,
                                                    as_str(): "FooBarData::<u32>::my_sub(50, 10)",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        1009,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 1332,
                                            end: 1455,
                                            as_str(): "fn my_sub(a: u64, b: u64) -> u64 {\n        if a >= b {\n            a - b\n        } else {\n            b - a\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3003,
                                                end: 3028,
                                                as_str(): "FooBarData::<u16>::my_sub",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3003,
                                    end: 3122,
                                    as_str(): "FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    )",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8004,
                            ),
                            type_ascription: TypeId(
                                8004,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 2995,
                    end: 3123,
                    as_str(): "let l = FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    );",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3132,
                                    end: 3133,
                                    as_str(): "m",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0f64dc000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/my_point.sw",
                                            ),
                                            start: 58,
                                            end: 65,
                                            as_str(): "MyPoint",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3154,
                                                    end: 3155,
                                                    as_str(): "x",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3157,
                                                    end: 3162,
                                                    as_str(): "10u64",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3172,
                                                    end: 3173,
                                                    as_str(): "y",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        10,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3175,
                                                    end: 3180,
                                                    as_str(): "10u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 3136,
                                        end: 3143,
                                        as_str(): "MyPoint",
                                    },
                                },
                                return_type: TypeId(
                                    8058,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3136,
                                    end: 3187,
                                    as_str(): "MyPoint {\n        x: 10u64,\n        y: 10u64,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8058,
                            ),
                            type_ascription: TypeId(
                                8055,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 3128,
                    end: 3188,
                    as_str(): "let m = MyPoint {\n        x: 10u64,\n        y: 10u64,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3197,
                                    end: 3198,
                                    as_str(): "n",
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3203,
                                                end: 3212,
                                                as_str(): "my_double",
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
                                                    src (ptr): 0x00007fe0f64dc000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/my_point.sw",
                                                    ),
                                                    start: 149,
                                                    end: 153,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3132,
                                                            end: 3133,
                                                            as_str(): "m",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3201,
                                                        end: 3202,
                                                        as_str(): "m",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    8058,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3201,
                                                    end: 3202,
                                                    as_str(): "m",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0f64dc000,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/my_point.sw",
                                                    ),
                                                    start: 155,
                                                    end: 160,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        100,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3213,
                                                    end: 3216,
                                                    as_str(): "100",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        1024,
                                        Span {
                                            src (ptr): 0x00007fe0f64dc000,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/my_point.sw",
                                            ),
                                            start: 136,
                                            end: 225,
                                            as_str(): "fn my_double(self, value: u64) -> u64 {\n        (self.x*2) + (self.y*2) + (value*2)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3203,
                                                end: 3212,
                                                as_str(): "my_double",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3201,
                                    end: 3217,
                                    as_str(): "m.my_double(100)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8062,
                            ),
                            type_ascription: TypeId(
                                8062,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 3193,
                    end: 3218,
                    as_str(): "let n = m.my_double(100);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3227,
                                    end: 3228,
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3233,
                                                end: 3242,
                                                as_str(): "my_triple",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2143,
                                                    end: 2147,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3132,
                                                            end: 3133,
                                                            as_str(): "m",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3231,
                                                        end: 3232,
                                                        as_str(): "m",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    8058,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3231,
                                                    end: 3232,
                                                    as_str(): "m",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2149,
                                                    end: 2154,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        100,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3243,
                                                    end: 3246,
                                                    as_str(): "100",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        1028,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2130,
                                            end: 2219,
                                            as_str(): "fn my_triple(self, value: u64) -> u64 {\n        (self.x*3) + (self.y*3) + (value*3)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3233,
                                                end: 3242,
                                                as_str(): "my_triple",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3231,
                                    end: 3247,
                                    as_str(): "m.my_triple(100)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8067,
                            ),
                            type_ascription: TypeId(
                                8067,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 3223,
                    end: 3248,
                    as_str(): "let o = m.my_triple(100);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3257,
                                    end: 3258,
                                    as_str(): "p",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2230,
                                            end: 2235,
                                            as_str(): "MyU64",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3277,
                                                    end: 3282,
                                                    as_str(): "inner",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3284,
                                                    end: 3289,
                                                    as_str(): "30u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 3261,
                                        end: 3266,
                                        as_str(): "MyU64",
                                    },
                                },
                                return_type: TypeId(
                                    7777,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3261,
                                    end: 3295,
                                    as_str(): "MyU64 {\n        inner: 30u64\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7777,
                            ),
                            type_ascription: TypeId(
                                8072,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 3253,
                    end: 3296,
                    as_str(): "let p = MyU64 {\n        inner: 30u64\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3305,
                                    end: 3306,
                                    as_str(): "q",
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
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3311,
                                                end: 3320,
                                                as_str(): "my_triple",
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
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2304,
                                                    end: 2308,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3257,
                                                            end: 3258,
                                                            as_str(): "p",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3309,
                                                        end: 3310,
                                                        as_str(): "p",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7777,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3309,
                                                    end: 3310,
                                                    as_str(): "p",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 2310,
                                                    end: 2315,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        1,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3321,
                                                    end: 3322,
                                                    as_str(): "1",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        1035,
                                        Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 2291,
                                            end: 2371,
                                            as_str(): "fn my_triple(self, value: u64) -> u64 {\n        (self.inner*3) + (value*3)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fd90ad70,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                ),
                                                start: 3311,
                                                end: 3320,
                                                as_str(): "my_triple",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3309,
                                    end: 3323,
                                    as_str(): "p.my_triple(1)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                8076,
                            ),
                            type_ascription: TypeId(
                                8076,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 3301,
                    end: 3324,
                    as_str(): "let q = p.my_triple(1);",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: LazyOperator {
                                    op: And,
                                    lhs: TyExpression {
                                        expression: LazyOperator {
                                            op: And,
                                            lhs: TyExpression {
                                                expression: LazyOperator {
                                                    op: And,
                                                    lhs: TyExpression {
                                                        expression: LazyOperator {
                                                            op: And,
                                                            lhs: TyExpression {
                                                                expression: LazyOperator {
                                                                    op: And,
                                                                    lhs: TyExpression {
                                                                        expression: LazyOperator {
                                                                            op: And,
                                                                            lhs: TyExpression {
                                                                                expression: LazyOperator {
                                                                                    op: And,
                                                                                    lhs: TyExpression {
                                                                                        expression: LazyOperator {
                                                                                            op: And,
                                                                                            lhs: TyExpression {
                                                                                                expression: LazyOperator {
                                                                                                    op: And,
                                                                                                    lhs: TyExpression {
                                                                                                        expression: FunctionApplication {
                                                                                                            call_path: CallPath {
                                                                                                                prefixes: [
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "core",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 3335,
                                                                                                                            end: 3337,
                                                                                                                            as_str(): "==",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: Some(
                                                                                                                            "ops",
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 3335,
                                                                                                                            end: 3337,
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
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 3335,
                                                                                                                        end: 3337,
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
                                                                                                                            src (ptr): 0x00007fe0f63e4490,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                            ),
                                                                                                                            start: 3297,
                                                                                                                            end: 3301,
                                                                                                                            as_str(): "self",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: VariableExpression {
                                                                                                                            name: BaseIdent {
                                                                                                                                name_override_opt: None,
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 2476,
                                                                                                                                    end: 2477,
                                                                                                                                    as_str(): "c",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 3333,
                                                                                                                                end: 3334,
                                                                                                                                as_str(): "c",
                                                                                                                            },
                                                                                                                            mutability: Immutable,
                                                                                                                        },
                                                                                                                        return_type: TypeId(
                                                                                                                            7861,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 3333,
                                                                                                                            end: 3334,
                                                                                                                            as_str(): "c",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                                (
                                                                                                                    BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0f63e4490,
                                                                                                                            path: Some(
                                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                            ),
                                                                                                                            start: 3303,
                                                                                                                            end: 3308,
                                                                                                                            as_str(): "other",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    TyExpression {
                                                                                                                        expression: Literal(
                                                                                                                            U8(
                                                                                                                                42,
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                        return_type: TypeId(
                                                                                                                            50,
                                                                                                                        ),
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 3338,
                                                                                                                            end: 3342,
                                                                                                                            as_str(): "42u8",
                                                                                                                        },
                                                                                                                    },
                                                                                                                ),
                                                                                                            ],
                                                                                                            function_decl_id: DeclId(
                                                                                                                1039,
                                                                                                                Span {
                                                                                                                    src (ptr): 0x00007fe0f63e4490,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                                                    ),
                                                                                                                    start: 3291,
                                                                                                                    end: 3357,
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
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 3335,
                                                                                                                        end: 3337,
                                                                                                                        as_str(): "==",
                                                                                                                    },
                                                                                                                },
                                                                                                            ),
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            71,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3333,
                                                                                                            end: 3342,
                                                                                                            as_str(): "c == 42u8",
                                                                                                        },
                                                                                                    },
                                                                                                    rhs: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 2497,
                                                                                                                    end: 2498,
                                                                                                                    as_str(): "d",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 3354,
                                                                                                                end: 3355,
                                                                                                                as_str(): "d",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            7866,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3354,
                                                                                                            end: 3355,
                                                                                                            as_str(): "d",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    71,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3333,
                                                                                                    end: 3355,
                                                                                                    as_str(): "c == 42u8\n        && d",
                                                                                                },
                                                                                            },
                                                                                            rhs: TyExpression {
                                                                                                expression: FunctionApplication {
                                                                                                    call_path: CallPath {
                                                                                                        prefixes: [
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "core",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3369,
                                                                                                                    end: 3371,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "ops",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3369,
                                                                                                                    end: 3371,
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
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 3369,
                                                                                                                end: 3371,
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
                                                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                                                expression: VariableExpression {
                                                                                                                    name: BaseIdent {
                                                                                                                        name_override_opt: None,
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 2528,
                                                                                                                            end: 2529,
                                                                                                                            as_str(): "e",
                                                                                                                        },
                                                                                                                        is_raw_ident: false,
                                                                                                                    },
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 3367,
                                                                                                                        end: 3368,
                                                                                                                        as_str(): "e",
                                                                                                                    },
                                                                                                                    mutability: Immutable,
                                                                                                                },
                                                                                                                return_type: TypeId(
                                                                                                                    7867,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3367,
                                                                                                                    end: 3368,
                                                                                                                    as_str(): "e",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                        (
                                                                                                            BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                                                        9,
                                                                                                                    ),
                                                                                                                ),
                                                                                                                return_type: TypeId(
                                                                                                                    21,
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 3372,
                                                                                                                    end: 3376,
                                                                                                                    as_str(): "9u64",
                                                                                                                },
                                                                                                            },
                                                                                                        ),
                                                                                                    ],
                                                                                                    function_decl_id: DeclId(
                                                                                                        1040,
                                                                                                        Span {
                                                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 3369,
                                                                                                                end: 3371,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    71,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3367,
                                                                                                    end: 3376,
                                                                                                    as_str(): "e == 9u64",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            71,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3333,
                                                                                            end: 3376,
                                                                                            as_str(): "c == 42u8\n        && d\n        && e == 9u64",
                                                                                        },
                                                                                    },
                                                                                    rhs: TyExpression {
                                                                                        expression: FunctionApplication {
                                                                                            call_path: CallPath {
                                                                                                prefixes: [
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "core",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3390,
                                                                                                            end: 3392,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "ops",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3390,
                                                                                                            end: 3392,
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
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3390,
                                                                                                        end: 3392,
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
                                                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 2611,
                                                                                                                    end: 2612,
                                                                                                                    as_str(): "g",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 3388,
                                                                                                                end: 3389,
                                                                                                                as_str(): "g",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            7886,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3388,
                                                                                                            end: 3389,
                                                                                                            as_str(): "g",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                (
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                                                10,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            21,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 3393,
                                                                                                            end: 3395,
                                                                                                            as_str(): "10",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                            ],
                                                                                            function_decl_id: DeclId(
                                                                                                1041,
                                                                                                Span {
                                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3390,
                                                                                                        end: 3392,
                                                                                                        as_str(): "==",
                                                                                                    },
                                                                                                },
                                                                                            ),
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            71,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3388,
                                                                                            end: 3395,
                                                                                            as_str(): "g == 10",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3333,
                                                                                    end: 3395,
                                                                                    as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10",
                                                                                },
                                                                            },
                                                                            rhs: TyExpression {
                                                                                expression: FunctionApplication {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "core",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3409,
                                                                                                    end: 3411,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3409,
                                                                                                    end: 3411,
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3409,
                                                                                                end: 3411,
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
                                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2673,
                                                                                                            end: 2674,
                                                                                                            as_str(): "h",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 3407,
                                                                                                        end: 3408,
                                                                                                        as_str(): "h",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7899,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3407,
                                                                                                    end: 3408,
                                                                                                    as_str(): "h",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                        (
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                                        50,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 3412,
                                                                                                    end: 3414,
                                                                                                    as_str(): "50",
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ],
                                                                                    function_decl_id: DeclId(
                                                                                        1042,
                                                                                        Span {
                                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3409,
                                                                                                end: 3411,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    71,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3407,
                                                                                    end: 3414,
                                                                                    as_str(): "h == 50",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3333,
                                                                            end: 3414,
                                                                            as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50",
                                                                        },
                                                                    },
                                                                    rhs: TyExpression {
                                                                        expression: FunctionApplication {
                                                                            call_path: CallPath {
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3428,
                                                                                            end: 3430,
                                                                                            as_str(): "==",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3428,
                                                                                            end: 3430,
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
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 3428,
                                                                                        end: 3430,
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
                                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                        expression: VariableExpression {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2937,
                                                                                                    end: 2938,
                                                                                                    as_str(): "k",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                                ),
                                                                                                start: 3426,
                                                                                                end: 3427,
                                                                                                as_str(): "k",
                                                                                            },
                                                                                            mutability: Immutable,
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7991,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3426,
                                                                                            end: 3427,
                                                                                            as_str(): "k",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                                10,
                                                                                            ),
                                                                                        ),
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 3431,
                                                                                            end: 3433,
                                                                                            as_str(): "10",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                1043,
                                                                                Span {
                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 3428,
                                                                                        end: 3430,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            71,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3426,
                                                                            end: 3433,
                                                                            as_str(): "k == 10",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3333,
                                                                    end: 3433,
                                                                    as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10",
                                                                },
                                                            },
                                                            rhs: TyExpression {
                                                                expression: FunctionApplication {
                                                                    call_path: CallPath {
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3447,
                                                                                    end: 3449,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3447,
                                                                                    end: 3449,
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3447,
                                                                                end: 3449,
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
                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                            ),
                                                                                            start: 2999,
                                                                                            end: 3000,
                                                                                            as_str(): "l",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                        ),
                                                                                        start: 3445,
                                                                                        end: 3446,
                                                                                        as_str(): "l",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    8004,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3445,
                                                                                    end: 3446,
                                                                                    as_str(): "l",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                                        50,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3450,
                                                                                    end: 3452,
                                                                                    as_str(): "50",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        1044,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3447,
                                                                                end: 3449,
                                                                                as_str(): "==",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    71,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3445,
                                                                    end: 3452,
                                                                    as_str(): "l == 50",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3333,
                                                            end: 3452,
                                                            as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50",
                                                        },
                                                    },
                                                    rhs: TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "core",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3466,
                                                                            end: 3468,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3466,
                                                                            end: 3468,
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
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3466,
                                                                        end: 3468,
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
                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                    ),
                                                                                    start: 3197,
                                                                                    end: 3198,
                                                                                    as_str(): "n",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                                ),
                                                                                start: 3464,
                                                                                end: 3465,
                                                                                as_str(): "n",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            8062,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3464,
                                                                            end: 3465,
                                                                            as_str(): "n",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                                240,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3469,
                                                                            end: 3472,
                                                                            as_str(): "240",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                1045,
                                                                Span {
                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3466,
                                                                        end: 3468,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3464,
                                                            end: 3472,
                                                            as_str(): "n == 240",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3333,
                                                    end: 3472,
                                                    as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240",
                                                },
                                            },
                                            rhs: TyExpression {
                                                expression: FunctionApplication {
                                                    call_path: CallPath {
                                                        prefixes: [
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "core",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3486,
                                                                    end: 3488,
                                                                    as_str(): "==",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3486,
                                                                    end: 3488,
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
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3486,
                                                                end: 3488,
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
                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe0fd90ad70,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                            ),
                                                                            start: 3227,
                                                                            end: 3228,
                                                                            as_str(): "o",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fd90ad70,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                        ),
                                                                        start: 3484,
                                                                        end: 3485,
                                                                        as_str(): "o",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    8067,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3484,
                                                                    end: 3485,
                                                                    as_str(): "o",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                                        360,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3489,
                                                                    end: 3492,
                                                                    as_str(): "360",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        1046,
                                                        Span {
                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3486,
                                                                end: 3488,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3484,
                                                    end: 3492,
                                                    as_str(): "o == 360",
                                                },
                                            },
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 3333,
                                            end: 3492,
                                            as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360",
                                        },
                                    },
                                    rhs: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3506,
                                                            end: 3508,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3506,
                                                            end: 3508,
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
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3506,
                                                        end: 3508,
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
                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fd90ad70,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                    ),
                                                                    start: 3305,
                                                                    end: 3306,
                                                                    as_str(): "q",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3504,
                                                                end: 3505,
                                                                as_str(): "q",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            8076,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3504,
                                                            end: 3505,
                                                            as_str(): "q",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0f63e4490,
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
                                                                93,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3509,
                                                            end: 3511,
                                                            as_str(): "93",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                1047,
                                                Span {
                                                    src (ptr): 0x00007fe0f63e4490,
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
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3506,
                                                        end: 3508,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0fd90ad70,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                            ),
                                            start: 3504,
                                            end: 3511,
                                            as_str(): "q == 93",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3333,
                                    end: 3511,
                                    as_str(): "c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: ImplicitReturnExpression(
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                42,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fd90ad70,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                            ),
                                                            start: 3522,
                                                            end: 3524,
                                                            as_str(): "42",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fd90ad70,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                    ),
                                                    start: 3522,
                                                    end: 3524,
                                                    as_str(): "42",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fd90ad70,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                    ),
                                    start: 3512,
                                    end: 3530,
                                    as_str(): "{\n        42\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [
                                                TyAstNode {
                                                    content: ImplicitReturnExpression(
                                                        TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    7,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fd90ad70,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                                ),
                                                                start: 3546,
                                                                end: 3547,
                                                                as_str(): "7",
                                                            },
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fd90ad70,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                                        ),
                                                        start: 3546,
                                                        end: 3547,
                                                        as_str(): "7",
                                                    },
                                                },
                                            ],
                                        },
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe0fd90ad70,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                                        ),
                                        start: 3536,
                                        end: 3553,
                                        as_str(): "{\n        7\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fd90ad70,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                            ),
                            start: 3330,
                            end: 3553,
                            as_str(): "if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fd90ad70,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
                    ),
                    start: 3330,
                    end: 3553,
                    as_str(): "if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 2375,
        end: 3555,
        as_str(): "fn main() -> u64 {\n    let a = FooBarData {\n        value: 1u8\n    };\n    let b = a.set(42);\n    let c = b.value;\n    let d = b.return_it(true);\n    let e = b.return_it(9u64);\n    let f = FooBarData {\n        value: 1u64\n    };\n    let g = f.my_add(a.my_add(1u8, 2u8), a.my_add(3u8, 4u8));\n    let h = FooBarData::<u64>::my_sub(\n        FooBarData::<u8>::my_sub(100, 10),\n        FooBarData::<u8>::my_sub(50, 10),\n    );\n    let i = OtherData {\n        a: true,\n        b: false,\n    };\n    let j = OtherData {\n        a: 10u32,\n        b: 11u32,\n    };\n    let k = j.my_add(i.my_add(1u8, 2u8), i.my_add(3u8, 4u8));\n    let l = FooBarData::<u16>::my_sub(\n        FooBarData::<u32>::my_sub(100, 10),\n        FooBarData::<u32>::my_sub(50, 10),\n    );\n    let m = MyPoint {\n        x: 10u64,\n        y: 10u64,\n    };\n    let n = m.my_double(100);\n    let o = m.my_triple(100);\n    let p = MyU64 {\n        inner: 30u64\n    };\n    let q = p.my_triple(1);\n\n    if c == 42u8\n        && d\n        && e == 9u64\n        && g == 10\n        && h == 50\n        && k == 10\n        && l == 50\n        && n == 240\n        && o == 360\n        && q == 93 {\n        42\n    } else {\n        7\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        21,
    ),
    initial_return_type: TypeId(
        21,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0fd90ad70,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRuBV0hp/generic_traits/src/main.sw",
        ),
        start: 2388,
        end: 2391,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

