
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 41,
            end: 46,
            as_str(): "MyAdd",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    interface_surface: [
        DeclId(
            13314,
            Span {
                src (ptr): 0x00007fe02ab473d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                ),
                start: 56,
                end: 62,
                as_str(): "my_add",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe02ab473d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
        ),
        start: 35,
        end: 92,
        as_str(): "trait MyAdd {\n    fn my_add(self, other: Self) -> Self;\n}",
    },
}
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 100,
            end: 105,
            as_str(): "MyMul",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    interface_surface: [
        DeclId(
            13317,
            Span {
                src (ptr): 0x00007fe02ab473d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                ),
                start: 115,
                end: 121,
                as_str(): "my_mul",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe02ab473d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
        ),
        start: 94,
        end: 151,
        as_str(): "trait MyMul {\n    fn my_mul(self, other: Self) -> Self;\n}",
    },
}
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 159,
            end: 165,
            as_str(): "MyMath",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    interface_surface: [],
    methods: [
        DeclId(
            13325,
            Span {
                src (ptr): 0x00007fe02ab473d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                ),
                start: 192,
                end: 252,
                as_str(): "fn my_double(self) -> Self {\n        self.my_add(self)\n    }",
            },
        ),
        DeclId(
            13329,
            Span {
                src (ptr): 0x00007fe02ab473d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                ),
                start: 258,
                end: 315,
                as_str(): "fn my_exp(self) -> Self {\n        self.my_mul(self)\n    }",
            },
        ),
    ],
    supertraits: [
        Supertrait {
            name: CallPath {
                prefixes: [],
                suffix: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 167,
                        end: 172,
                        as_str(): "MyAdd",
                    },
                    is_raw_ident: false,
                },
                is_absolute: false,
            },
        },
        Supertrait {
            name: CallPath {
                prefixes: [],
                suffix: BaseIdent {
                    name_override_opt: None,
                    span: Span {
                        src (ptr): 0x00007fe02ab473d0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                        ),
                        start: 175,
                        end: 180,
                        as_str(): "MyMul",
                    },
                    is_raw_ident: false,
                },
                is_absolute: false,
            },
        },
    ],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe02ab473d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
        ),
        start: 153,
        end: 317,
        as_str(): "trait MyMath: MyAdd + MyMul {\n\n} {\n    fn my_double(self) -> Self {\n        self.my_add(self)\n    }\n\n    fn my_exp(self) -> Self {\n        self.my_mul(self)\n    }\n}",
    },
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 326,
            end: 330,
            as_str(): "Data",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe02ab473d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                    ),
                    start: 337,
                    end: 342,
                    as_str(): "value",
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
                src (ptr): 0x00007fe02ab473d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                ),
                start: 337,
                end: 347,
                as_str(): "value: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe02ab473d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                ),
                start: 344,
                end: 347,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe02ab473d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
        ),
        start: 319,
        end: 350,
        as_str(): "struct Data {\n    value: u64,\n}",
    },
    attributes: {},
}
ImplTrait(
    DeclId(
        13334,
        Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 352,
            end: 493,
            as_str(): "impl MyAdd for Data {\n    fn my_add(self, other: Self) -> Self {\n        Data {\n            value: self.value + other.value\n        }\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        13337,
        Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 495,
            end: 636,
            as_str(): "impl MyMul for Data {\n    fn my_mul(self, other: Self) -> Self {\n        Data {\n            value: self.value * other.value\n        }\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        13348,
        Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 638,
            end: 661,
            as_str(): "impl MyMath for Data {}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe02ab473d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
            ),
            start: 666,
            end: 670,
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
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 691,
                                    end: 692,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 326,
                                            end: 330,
                                            as_str(): "Data",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 710,
                                                    end: 715,
                                                    as_str(): "value",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        3,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 717,
                                                    end: 721,
                                                    as_str(): "3u64",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 695,
                                        end: 699,
                                        as_str(): "Data",
                                    },
                                },
                                return_type: TypeId(
                                    31776,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 695,
                                    end: 727,
                                    as_str(): "Data {\n        value: 3u64\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31776,
                            ),
                            type_ascription: TypeId(
                                31997,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe02ab473d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                    ),
                    start: 687,
                    end: 728,
                    as_str(): "let a = Data {\n        value: 3u64\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 737,
                                    end: 738,
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
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 743,
                                                end: 749,
                                                as_str(): "my_exp",
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
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 268,
                                                    end: 272,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 691,
                                                            end: 692,
                                                            as_str(): "a",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 741,
                                                        end: 742,
                                                        as_str(): "a",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31776,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 741,
                                                    end: 742,
                                                    as_str(): "a",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13349,
                                        Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 258,
                                            end: 315,
                                            as_str(): "fn my_exp(self) -> Self {\n        self.my_mul(self)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 743,
                                                end: 749,
                                                as_str(): "my_exp",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31776,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 741,
                                    end: 751,
                                    as_str(): "a.my_exp()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31776,
                            ),
                            type_ascription: TypeId(
                                32000,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe02ab473d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                    ),
                    start: 733,
                    end: 752,
                    as_str(): "let b = a.my_exp();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 761,
                                    end: 762,
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
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 767,
                                                end: 776,
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
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 205,
                                                    end: 209,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 737,
                                                            end: 738,
                                                            as_str(): "b",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 765,
                                                        end: 766,
                                                        as_str(): "b",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    31776,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe02ab473d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                    ),
                                                    start: 765,
                                                    end: 766,
                                                    as_str(): "b",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13350,
                                        Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 192,
                                            end: 252,
                                            as_str(): "fn my_double(self) -> Self {\n        self.my_add(self)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe02ab473d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                ),
                                                start: 767,
                                                end: 776,
                                                as_str(): "my_double",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31776,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe02ab473d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                    ),
                                    start: 765,
                                    end: 778,
                                    as_str(): "b.my_double()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31776,
                            ),
                            type_ascription: TypeId(
                                32002,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe02ab473d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                    ),
                    start: 757,
                    end: 779,
                    as_str(): "let c = b.my_double();",
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 784,
                                        end: 790,
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
                                            src (ptr): 0x00007fe03dfd8410,
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
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 799,
                                                            end: 801,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 799,
                                                            end: 801,
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
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 799,
                                                        end: 801,
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
                                                            src (ptr): 0x00007fe043facf90,
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
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe02ab473d0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                            ),
                                                                            start: 761,
                                                                            end: 762,
                                                                            as_str(): "c",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 791,
                                                                        end: 792,
                                                                        as_str(): "c",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31776,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 791,
                                                                    end: 792,
                                                                    as_str(): "c",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe02ab473d0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                        ),
                                                                        start: 337,
                                                                        end: 342,
                                                                        as_str(): "value",
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
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 337,
                                                                    end: 347,
                                                                    as_str(): "value: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe02ab473d0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                    ),
                                                                    start: 344,
                                                                    end: 347,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fe02ab473d0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                                ),
                                                                start: 793,
                                                                end: 798,
                                                                as_str(): "value",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                31776,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 791,
                                                            end: 798,
                                                            as_str(): "c.value",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe043facf90,
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
                                                                18,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe02ab473d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                            ),
                                                            start: 802,
                                                            end: 804,
                                                            as_str(): "18",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13352,
                                                Span {
                                                    src (ptr): 0x00007fe043facf90,
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
                                                        src (ptr): 0x00007fe02ab473d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                                        ),
                                                        start: 799,
                                                        end: 801,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe02ab473d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                            ),
                                            start: 791,
                                            end: 804,
                                            as_str(): "c.value == 18",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13353,
                                Span {
                                    src (ptr): 0x00007fe03dfd8410,
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
                                        src (ptr): 0x00007fe02ab473d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                                        ),
                                        start: 784,
                                        end: 790,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            32010,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 784,
                            end: 805,
                            as_str(): "assert(c.value == 18)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe02ab473d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                    ),
                    start: 784,
                    end: 805,
                    as_str(): "assert(c.value == 18)",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
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
                            src (ptr): 0x00007fe02ab473d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                            ),
                            start: 812,
                            end: 816,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe02ab473d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
                    ),
                    start: 812,
                    end: 816,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe02ab473d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
        ),
        start: 663,
        end: 818,
        as_str(): "fn main() -> bool {\n    let a = Data {\n        value: 3u64\n    };\n    let b = a.my_exp();\n    let c = b.my_double();\n    assert(c.value == 18);\n\n    true\n}",
    },
    attributes: {},
    return_type: TypeId(
        71,
    ),
    initial_return_type: TypeId(
        71,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe02ab473d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR03Xo6Y/supertraits_with_trait_methods/src/main.sw",
        ),
        start: 676,
        end: 680,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

