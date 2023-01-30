TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 876,
            end: 884,
            as_str(): "MyStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 891,
                    end: 892,
                    as_str(): "a",
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
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 891,
                end: 897,
                as_str(): "a: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 894,
                end: 897,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb13d3811d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
        ),
        start: 869,
        end: 900,
        as_str(): "struct MyStruct {\n    a: u64,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 907,
            end: 913,
            as_str(): "MyEnum",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 920,
                    end: 926,
                    as_str(): "Number",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 928,
                end: 931,
                as_str(): "u64",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 920,
                end: 931,
                as_str(): "Number: u64",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 937,
                    end: 941,
                    as_str(): "Unit",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7252,
            ),
            initial_type_id: TypeId(
                7251,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 943,
                end: 945,
                as_str(): "()",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 937,
                end: 945,
                as_str(): "Unit: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb13d3811d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
        ),
        start: 902,
        end: 948,
        as_str(): "enum MyEnum {\n    Number: u64,\n    Unit: (),\n}",
    },
    visibility: Private,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 957,
            end: 973,
            as_str(): "MyStructWithEnum",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 980,
                    end: 981,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7254,
            ),
            initial_type_id: TypeId(
                7253,
            ),
            span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 980,
                end: 991,
                as_str(): "a: MyStruct",
            },
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 983,
                end: 991,
                as_str(): "MyStruct",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 997,
                    end: 998,
                    as_str(): "b",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7256,
            ),
            initial_type_id: TypeId(
                7255,
            ),
            span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 997,
                end: 1006,
                as_str(): "b: MyEnum",
            },
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1000,
                end: 1006,
                as_str(): "MyEnum",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb13d3811d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
        ),
        start: 950,
        end: 1009,
        as_str(): "struct MyStructWithEnum {\n    a: MyStruct,\n    b: MyEnum,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 1014,
            end: 1024,
            as_str(): "eight_args",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
                            TyExpression {
                                expression: Tuple {
                                    fields: [],
                                },
                                return_type: TypeId(
                                    7268,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1120,
                                    end: 1126,
                                    as_str(): "return",
                                },
                            },
                        ),
                        return_type: TypeId(
                            7269,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 1120,
                            end: 1126,
                            as_str(): "return",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1120,
                    end: 1126,
                    as_str(): "return",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1025,
                    end: 1026,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                7254,
            ),
            initial_type_id: TypeId(
                7258,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1028,
                end: 1036,
                as_str(): "MyStruct",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1038,
                    end: 1039,
                    as_str(): "b",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                7256,
            ),
            initial_type_id: TypeId(
                7259,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1041,
                end: 1047,
                as_str(): "MyEnum",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1049,
                    end: 1050,
                    as_str(): "c",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                7261,
            ),
            initial_type_id: TypeId(
                7260,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1052,
                end: 1068,
                as_str(): "MyStructWithEnum",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1070,
                    end: 1071,
                    as_str(): "d",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                7262,
            ),
            initial_type_id: TypeId(
                7262,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1073,
                end: 1079,
                as_str(): "str[5]",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1081,
                    end: 1082,
                    as_str(): "e",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1084,
                end: 1088,
                as_str(): "bool",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1090,
                    end: 1091,
                    as_str(): "f",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1093,
                end: 1096,
                as_str(): "u64",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1098,
                    end: 1099,
                    as_str(): "g",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                50,
            ),
            initial_type_id: TypeId(
                50,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1101,
                end: 1103,
                as_str(): "u8",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 1105,
                    end: 1106,
                    as_str(): "h",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fb14c011bb0,
                path: None,
                start: 0,
                end: 0,
                as_str(): "",
            },
            type_id: TypeId(
                59,
            ),
            initial_type_id: TypeId(
                59,
            ),
            type_span: Span {
                src (ptr): 0x00007fb13d3811d0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                ),
                start: 1108,
                end: 1112,
                as_str(): "b256",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb13d3811d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
        ),
        start: 1011,
        end: 1129,
        as_str(): "fn eight_args(a: MyStruct, b: MyEnum, c: MyStructWithEnum, d: str[5], e: bool, f: u64, g: u8, h: b256) {\n    return;\n}",
    },
    attributes: {},
    return_type: TypeId(
        7264,
    ),
    initial_return_type: TypeId(
        7263,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb13d3811d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
        ),
        start: 1011,
        end: 1113,
        as_str(): "fn eight_args(a: MyStruct, b: MyEnum, c: MyStructWithEnum, d: str[5], e: bool, f: u64, g: u8, h: b256)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb13d3811d0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
            ),
            start: 71,
            end: 75,
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
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 96,
                                    end: 105,
                                    as_str(): "my_struct",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 876,
                                            end: 884,
                                            as_str(): "MyStruct",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 127,
                                                    end: 128,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 130,
                                                    end: 131,
                                                    as_str(): "5",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 108,
                                        end: 116,
                                        as_str(): "MyStruct",
                                    },
                                },
                                return_type: TypeId(
                                    7254,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 108,
                                    end: 138,
                                    as_str(): "MyStruct {\n        a: 5,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7254,
                            ),
                            type_ascription: TypeId(
                                7271,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 92,
                    end: 139,
                    as_str(): "let my_struct = MyStruct {\n        a: 5,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 148,
                                    end: 155,
                                    as_str(): "my_enum",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 907,
                                                end: 913,
                                                as_str(): "MyEnum",
                                            },
                                            is_raw_ident: false,
                                        },
                                        type_parameters: [],
                                        attributes: {},
                                        variants: [
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 920,
                                                        end: 926,
                                                        as_str(): "Number",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    21,
                                                ),
                                                initial_type_id: TypeId(
                                                    21,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 928,
                                                    end: 931,
                                                    as_str(): "u64",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 920,
                                                    end: 931,
                                                    as_str(): "Number: u64",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 937,
                                                        end: 941,
                                                        as_str(): "Unit",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7252,
                                                ),
                                                initial_type_id: TypeId(
                                                    7251,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 943,
                                                    end: 945,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 937,
                                                    end: 945,
                                                    as_str(): "Unit: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 902,
                                            end: 948,
                                            as_str(): "enum MyEnum {\n    Number: u64,\n    Unit: (),\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 920,
                                            end: 926,
                                            as_str(): "Number",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
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
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 173,
                                                end: 175,
                                                as_str(): "10",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 158,
                                        end: 164,
                                        as_str(): "MyEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 166,
                                        end: 172,
                                        as_str(): "Number",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 158,
                                            end: 172,
                                            as_str(): "MyEnum::Number",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7256,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 166,
                                    end: 172,
                                    as_str(): "Number",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7256,
                            ),
                            type_ascription: TypeId(
                                7275,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 144,
                    end: 177,
                    as_str(): "let my_enum = MyEnum::Number(10);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 186,
                                    end: 205,
                                    as_str(): "my_struct_with_enum",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 957,
                                            end: 973,
                                            as_str(): "MyStructWithEnum",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 235,
                                                    end: 236,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 96,
                                                            end: 105,
                                                            as_str(): "my_struct",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 238,
                                                        end: 247,
                                                        as_str(): "my_struct",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7254,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 238,
                                                    end: 247,
                                                    as_str(): "my_struct",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 257,
                                                    end: 258,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb13d3811d0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                            ),
                                                            start: 148,
                                                            end: 155,
                                                            as_str(): "my_enum",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fb13d3811d0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                        ),
                                                        start: 260,
                                                        end: 267,
                                                        as_str(): "my_enum",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7256,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 260,
                                                    end: 267,
                                                    as_str(): "my_enum",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 208,
                                        end: 224,
                                        as_str(): "MyStructWithEnum",
                                    },
                                },
                                return_type: TypeId(
                                    7261,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 208,
                                    end: 274,
                                    as_str(): "MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7261,
                            ),
                            type_ascription: TypeId(
                                7278,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 182,
                    end: 275,
                    as_str(): "let my_struct_with_enum = MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 284,
                                    end: 285,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    String(
                                        Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 289,
                                            end: 294,
                                            as_str(): "abcde",
                                        },
                                    ),
                                ),
                                return_type: TypeId(
                                    7283,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 288,
                                    end: 295,
                                    as_str(): "\"abcde\"",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7283,
                            ),
                            type_ascription: TypeId(
                                7282,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 280,
                    end: 296,
                    as_str(): "let d = \"abcde\";",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 305,
                                    end: 306,
                                    as_str(): "e",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 309,
                                    end: 313,
                                    as_str(): "true",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7284,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 301,
                    end: 314,
                    as_str(): "let e = true;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 323,
                                    end: 324,
                                    as_str(): "f",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        15,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 327,
                                    end: 329,
                                    as_str(): "15",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7285,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 319,
                    end: 330,
                    as_str(): "let f = 15;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 339,
                                    end: 340,
                                    as_str(): "g",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        170,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 343,
                                    end: 353,
                                    as_str(): "0b10101010",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7287,
                            ),
                            type_ascription: TypeId(
                                7287,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 335,
                    end: 354,
                    as_str(): "let g = 0b10101010;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 363,
                                    end: 364,
                                    as_str(): "h",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    B256(
                                        [
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                            170,
                                        ],
                                    ),
                                ),
                                return_type: TypeId(
                                    59,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 373,
                                    end: 631,
                                    as_str(): "0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                59,
                            ),
                            type_ascription: TypeId(
                                59,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 366,
                                    end: 370,
                                    as_str(): "b256",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 359,
                    end: 632,
                    as_str(): "let h: b256 = 0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010;",
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
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 638,
                                        end: 648,
                                        as_str(): "eight_args",
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
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1025,
                                            end: 1026,
                                            as_str(): "a",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 96,
                                                    end: 105,
                                                    as_str(): "my_struct",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 649,
                                                end: 658,
                                                as_str(): "my_struct",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7254,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 649,
                                            end: 658,
                                            as_str(): "my_struct",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1038,
                                            end: 1039,
                                            as_str(): "b",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 148,
                                                    end: 155,
                                                    as_str(): "my_enum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 660,
                                                end: 667,
                                                as_str(): "my_enum",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7256,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 660,
                                            end: 667,
                                            as_str(): "my_enum",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1049,
                                            end: 1050,
                                            as_str(): "c",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 186,
                                                    end: 205,
                                                    as_str(): "my_struct_with_enum",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 669,
                                                end: 688,
                                                as_str(): "my_struct_with_enum",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7261,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 669,
                                            end: 688,
                                            as_str(): "my_struct_with_enum",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1070,
                                            end: 1071,
                                            as_str(): "d",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 284,
                                                    end: 285,
                                                    as_str(): "d",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 690,
                                                end: 691,
                                                as_str(): "d",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7283,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 690,
                                            end: 691,
                                            as_str(): "d",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1081,
                                            end: 1082,
                                            as_str(): "e",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 305,
                                                    end: 306,
                                                    as_str(): "e",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 693,
                                                end: 694,
                                                as_str(): "e",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 693,
                                            end: 694,
                                            as_str(): "e",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1090,
                                            end: 1091,
                                            as_str(): "f",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 323,
                                                    end: 324,
                                                    as_str(): "f",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 696,
                                                end: 697,
                                                as_str(): "f",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 696,
                                            end: 697,
                                            as_str(): "f",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1098,
                                            end: 1099,
                                            as_str(): "g",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 339,
                                                    end: 340,
                                                    as_str(): "g",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 699,
                                                end: 700,
                                                as_str(): "g",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            7287,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 699,
                                            end: 700,
                                            as_str(): "g",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 1105,
                                            end: 1106,
                                            as_str(): "h",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 363,
                                                    end: 364,
                                                    as_str(): "h",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 702,
                                                end: 703,
                                                as_str(): "h",
                                            },
                                            mutability: Immutable,
                                        },
                                        return_type: TypeId(
                                            59,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb13d3811d0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                            ),
                                            start: 702,
                                            end: 703,
                                            as_str(): "h",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                550,
                                Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 1011,
                                    end: 1129,
                                    as_str(): "fn eight_args(a: MyStruct, b: MyEnum, c: MyStructWithEnum, d: str[5], e: bool, f: u64, g: u8, h: b256) {\n    return;\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb13d3811d0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                        ),
                                        start: 638,
                                        end: 648,
                                        as_str(): "eight_args",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7297,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 638,
                            end: 704,
                            as_str(): "eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 638,
                    end: 704,
                    as_str(): "eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 744,
                                    end: 751,
                                    as_str(): "ls_than",
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
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 756,
                                                    end: 757,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 756,
                                                    end: 757,
                                                    as_str(): "<",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "lt",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 756,
                                                end: 757,
                                                as_str(): "<",
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
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 4000,
                                                    end: 4004,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 754,
                                                    end: 755,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 4006,
                                                    end: 4011,
                                                    as_str(): "other",
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
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 758,
                                                    end: 759,
                                                    as_str(): "5",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        551,
                                        Span {
                                            src (ptr): 0x00007fb139eb6f50,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3994,
                                            end: 4129,
                                            as_str(): "fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 756,
                                                end: 757,
                                                as_str(): "<",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 754,
                                    end: 759,
                                    as_str(): "4 < 5",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7298,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 740,
                    end: 760,
                    as_str(): "let ls_than = 4 < 5;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 769,
                                    end: 776,
                                    as_str(): "gt_than",
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
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 781,
                                                    end: 782,
                                                    as_str(): ">",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 781,
                                                    end: 782,
                                                    as_str(): ">",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "gt",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 781,
                                                end: 782,
                                                as_str(): ">",
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
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3860,
                                                    end: 3864,
                                                    as_str(): "self",
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
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 779,
                                                    end: 780,
                                                    as_str(): "5",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3866,
                                                    end: 3871,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 783,
                                                    end: 784,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        552,
                                        Span {
                                            src (ptr): 0x00007fb139eb6f50,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 3854,
                                            end: 3989,
                                            as_str(): "fn gt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 781,
                                                end: 782,
                                                as_str(): ">",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 779,
                                    end: 784,
                                    as_str(): "5 > 4",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7303,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 765,
                    end: 785,
                    as_str(): "let gt_than = 5 > 4;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 794,
                                    end: 796,
                                    as_str(): "le",
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
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 801,
                                                    end: 803,
                                                    as_str(): "<=",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 801,
                                                    end: 803,
                                                    as_str(): "<=",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "le",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 801,
                                                end: 803,
                                                as_str(): "<=",
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
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 11136,
                                                    end: 11140,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 799,
                                                    end: 800,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 11142,
                                                    end: 11147,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 804,
                                                    end: 805,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        553,
                                        Span {
                                            src (ptr): 0x00007fb139eb6f50,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 11130,
                                            end: 11211,
                                            as_str(): "fn le(self, other: Self) -> bool {\n        self.lt(other) || self.eq(other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 801,
                                                end: 803,
                                                as_str(): "<=",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 799,
                                    end: 805,
                                    as_str(): "4 <= 4",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7308,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 790,
                    end: 806,
                    as_str(): "let le = 4 <= 4;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 815,
                                    end: 817,
                                    as_str(): "ge",
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
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 822,
                                                    end: 824,
                                                    as_str(): ">=",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 822,
                                                    end: 824,
                                                    as_str(): ">=",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "ge",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 822,
                                                end: 824,
                                                as_str(): ">=",
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
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 11050,
                                                    end: 11054,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 820,
                                                    end: 821,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb139eb6f50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 11056,
                                                    end: 11061,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: Literal(
                                                    U64(
                                                        4,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 825,
                                                    end: 826,
                                                    as_str(): "4",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        554,
                                        Span {
                                            src (ptr): 0x00007fb139eb6f50,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                            ),
                                            start: 11044,
                                            end: 11125,
                                            as_str(): "fn ge(self, other: Self) -> bool {\n        self.gt(other) || self.eq(other)\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 822,
                                                end: 824,
                                                as_str(): ">=",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 820,
                                    end: 826,
                                    as_str(): "4 >= 4",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7313,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 811,
                    end: 827,
                    as_str(): "let ge = 4 >= 4;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 836,
                                    end: 838,
                                    as_str(): "eq",
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
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 843,
                                                    end: 845,
                                                    as_str(): "==",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 843,
                                                    end: 845,
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
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 843,
                                                end: 845,
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
                                                    src (ptr): 0x00007fb139eb6f50,
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
                                                expression: Literal(
                                                    U64(
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 841,
                                                    end: 842,
                                                    as_str(): "5",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb139eb6f50,
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
                                                        5,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb13d3811d0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                    ),
                                                    start: 846,
                                                    end: 847,
                                                    as_str(): "5",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        555,
                                        Span {
                                            src (ptr): 0x00007fb139eb6f50,
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
                                                src (ptr): 0x00007fb13d3811d0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                                ),
                                                start: 843,
                                                end: 845,
                                                as_str(): "==",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 841,
                                    end: 847,
                                    as_str(): "5 == 5",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7318,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 832,
                    end: 848,
                    as_str(): "let eq = 5 == 5;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
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
                                    src (ptr): 0x00007fb13d3811d0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                                    ),
                                    start: 861,
                                    end: 865,
                                    as_str(): "true",
                                },
                            },
                        ),
                        return_type: TypeId(
                            7325,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb13d3811d0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                            ),
                            start: 854,
                            end: 865,
                            as_str(): "return true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb13d3811d0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
                    ),
                    start: 854,
                    end: 865,
                    as_str(): "return true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb13d3811d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
        ),
        start: 68,
        end: 868,
        as_str(): "fn main() -> bool {\n    let my_struct = MyStruct {\n        a: 5,\n    };\n    let my_enum = MyEnum::Number(10);\n    let my_struct_with_enum = MyStructWithEnum {\n        a: my_struct,\n        b: my_enum,\n    };\n    let d = \"abcde\";\n    let e = true;\n    let f = 15;\n    let g = 0b10101010;\n    let h: b256 = 0b1010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010101010;\n\n    eight_args(my_struct, my_enum, my_struct_with_enum, d, e, f, g, h);\n\n    // test some comparisons\n    let ls_than = 4 < 5;\n    let gt_than = 5 > 4;\n    let le = 4 <= 4;\n    let ge = 4 >= 4;\n    let eq = 5 == 5;\n\n    return true;\n}",
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
        src (ptr): 0x00007fb13d3811d0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRHmLMMd/basic_func_decl/src/main.sw",
        ),
        start: 81,
        end: 85,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

