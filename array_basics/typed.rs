TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 16,
            end: 17,
            as_str(): "S",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 24,
                    end: 27,
                    as_str(): "foo",
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
                src (ptr): 0x00007fb14c154030,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                ),
                start: 24,
                end: 32,
                as_str(): "foo: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb14c154030,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                ),
                start: 29,
                end: 32,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 38,
                    end: 41,
                    as_str(): "bar",
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
                src (ptr): 0x00007fb14c154030,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                ),
                start: 38,
                end: 46,
                as_str(): "bar: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fb14c154030,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                ),
                start: 43,
                end: 46,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fb14c154030,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
        ),
        start: 9,
        end: 49,
        as_str(): "struct S {\n    foo: u64,\n    bar: u64,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 563,
            end: 564,
            as_str(): "i",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Array {
                            contents: [
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
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 586,
                                        end: 587,
                                        as_str(): "0",
                                    },
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
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 589,
                                        end: 590,
                                        as_str(): "1",
                                    },
                                },
                                TyExpression {
                                    expression: Literal(
                                        U64(
                                            2,
                                        ),
                                    ),
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 592,
                                        end: 593,
                                        as_str(): "2",
                                    },
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
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 595,
                                        end: 596,
                                        as_str(): "3",
                                    },
                                },
                            ],
                        },
                        return_type: TypeId(
                            7265,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 585,
                            end: 597,
                            as_str(): "[0, 1, 2, 3]",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 585,
                    end: 597,
                    as_str(): "[0, 1, 2, 3]",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14c154030,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
        ),
        start: 560,
        end: 599,
        as_str(): "fn i() -> [u64;\n4] {\n    [0, 1, 2, 3]\n}",
    },
    attributes: {},
    return_type: TypeId(
        7255,
    ),
    initial_return_type: TypeId(
        7254,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb14c154030,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
        ),
        start: 570,
        end: 578,
        as_str(): "[u64;\n4]",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 604,
            end: 605,
            as_str(): "j",
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
                                        name_override_opt: Some(
                                            "core",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 669,
                                            end: 671,
                                            as_str(): "==",
                                        },
                                        is_raw_ident: false,
                                    },
                                    BaseIdent {
                                        name_override_opt: Some(
                                            "ops",
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 669,
                                            end: 671,
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
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 669,
                                        end: 671,
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
                                            src (ptr): 0x00007fb1451bedb0,
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
                                                prefixes: [
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "core",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 653,
                                                            as_str(): "+",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 653,
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
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 652,
                                                        end: 653,
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
                                                            src (ptr): 0x00007fb1451bedb0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 124,
                                                            end: 128,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: ArrayIndex {
                                                                    prefix: TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 606,
                                                                                    end: 613,
                                                                                    as_str(): "ary_arg",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 637,
                                                                                end: 644,
                                                                                as_str(): "ary_arg",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7275,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 637,
                                                                            end: 644,
                                                                            as_str(): "ary_arg",
                                                                        },
                                                                    },
                                                                    index: TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            7276,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 645,
                                                                            end: 646,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    7268,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 637,
                                                                    end: 647,
                                                                    as_str(): "ary_arg[0]",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 24,
                                                                        end: 27,
                                                                        as_str(): "foo",
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
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 24,
                                                                    end: 32,
                                                                    as_str(): "foo: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 29,
                                                                    end: 32,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 648,
                                                                end: 651,
                                                                as_str(): "foo",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                7268,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 637,
                                                            end: 651,
                                                            as_str(): "ary_arg[0].foo",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb1451bedb0,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 130,
                                                            end: 135,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: StructFieldAccess {
                                                            prefix: TyExpression {
                                                                expression: ArrayIndex {
                                                                    prefix: TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 606,
                                                                                    end: 613,
                                                                                    as_str(): "ary_arg",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 654,
                                                                                end: 661,
                                                                                as_str(): "ary_arg",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            7281,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 654,
                                                                            end: 661,
                                                                            as_str(): "ary_arg",
                                                                        },
                                                                    },
                                                                    index: TyExpression {
                                                                        expression: Literal(
                                                                            U64(
                                                                                1,
                                                                            ),
                                                                        ),
                                                                        return_type: TypeId(
                                                                            7282,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 662,
                                                                            end: 663,
                                                                            as_str(): "1",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    7268,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 654,
                                                                    end: 664,
                                                                    as_str(): "ary_arg[1]",
                                                                },
                                                            },
                                                            field_to_access: TyStructField {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 38,
                                                                        end: 41,
                                                                        as_str(): "bar",
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
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 38,
                                                                    end: 46,
                                                                    as_str(): "bar: u64",
                                                                },
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 43,
                                                                    end: 46,
                                                                    as_str(): "u64",
                                                                },
                                                                attributes: {},
                                                            },
                                                            field_instantiation_span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 665,
                                                                end: 668,
                                                                as_str(): "bar",
                                                            },
                                                            resolved_type_of_parent: TypeId(
                                                                7268,
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 654,
                                                            end: 668,
                                                            as_str(): "ary_arg[1].bar",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                546,
                                                Span {
                                                    src (ptr): 0x00007fb1451bedb0,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 117,
                                                    end: 185,
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
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 652,
                                                        end: 653,
                                                        as_str(): "+",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 637,
                                            end: 668,
                                            as_str(): "ary_arg[0].foo + ary_arg[1].bar",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb1451bedb0,
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
                                                12,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 672,
                                            end: 674,
                                            as_str(): "12",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                547,
                                Span {
                                    src (ptr): 0x00007fb1451bedb0,
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
                                        src (ptr): 0x00007fb14c154030,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                        ),
                                        start: 669,
                                        end: 671,
                                        as_str(): "==",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 637,
                            end: 674,
                            as_str(): "ary_arg[0].foo + ary_arg[1].bar == 12",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 637,
                    end: 674,
                    as_str(): "ary_arg[0].foo + ary_arg[1].bar == 12",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 606,
                    end: 613,
                    as_str(): "ary_arg",
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
                7269,
            ),
            initial_type_id: TypeId(
                7267,
            ),
            type_span: Span {
                src (ptr): 0x00007fb14c154030,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                ),
                start: 615,
                end: 621,
                as_str(): "[S;\n2]",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14c154030,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
        ),
        start: 601,
        end: 676,
        as_str(): "fn j(ary_arg: [S;\n2]) -> bool {\n    ary_arg[0].foo + ary_arg[1].bar == 12\n}",
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
        src (ptr): 0x00007fb14c154030,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
        ),
        start: 626,
        end: 630,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14c154030,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
            ),
            start: 54,
            end: 58,
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
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 79,
                                    end: 80,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 99,
                                                end: 103,
                                                as_str(): "true",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 105,
                                                end: 109,
                                                as_str(): "true",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 111,
                                                end: 115,
                                                as_str(): "true",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 117,
                                                end: 122,
                                                as_str(): "false",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 124,
                                                end: 128,
                                                as_str(): "true",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7294,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 98,
                                    end: 129,
                                    as_str(): "[true, true, true, false, true]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7294,
                            ),
                            type_ascription: TypeId(
                                7287,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 82,
                                    end: 95,
                                    as_str(): "[bool;\n    5]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 75,
                    end: 130,
                    as_str(): "let a: [bool;\n    5] = [true, true, true, false, true];",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 139,
                                    end: 140,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 159,
                                                end: 160,
                                                as_str(): "3",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7318,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 158,
                                    end: 169,
                                    as_str(): "[3;\n    10]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7318,
                            ),
                            type_ascription: TypeId(
                                7296,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 142,
                                    end: 155,
                                    as_str(): "[u32;\n    10]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 135,
                    end: 170,
                    as_str(): "let b: [u32;\n    10] = [3;\n    10];",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 179,
                                    end: 180,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 184,
                                                end: 188,
                                                as_str(): "0x01",
                                            },
                                        },
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    2,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 190,
                                                end: 194,
                                                as_str(): "0x02",
                                            },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 196,
                                                end: 200,
                                                as_str(): "0x03",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7324,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 183,
                                    end: 201,
                                    as_str(): "[0x01, 0x02, 0x03]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7324,
                            ),
                            type_ascription: TypeId(
                                7319,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 175,
                    end: 202,
                    as_str(): "let c = [0x01, 0x02, 0x03];",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 211,
                                    end: 212,
                                    as_str(): "d",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
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
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 216,
                                                end: 217,
                                                as_str(): "0",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7347,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 215,
                                    end: 226,
                                    as_str(): "[0;\n    10]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7347,
                            ),
                            type_ascription: TypeId(
                                7325,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 207,
                    end: 227,
                    as_str(): "let d = [0;\n    10];",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 236,
                                    end: 237,
                                    as_str(): "e",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: Array {
                                                contents: [
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
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 265,
                                                            end: 266,
                                                            as_str(): "1",
                                                        },
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                2,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 268,
                                                            end: 269,
                                                            as_str(): "2",
                                                        },
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
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 271,
                                                            end: 272,
                                                            as_str(): "3",
                                                        },
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
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 274,
                                                            end: 275,
                                                            as_str(): "4",
                                                        },
                                                    },
                                                ],
                                            },
                                            return_type: TypeId(
                                                7361,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 264,
                                                end: 276,
                                                as_str(): "[1, 2, 3, 4]",
                                            },
                                        },
                                        TyExpression {
                                            expression: Array {
                                                contents: [
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
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 279,
                                                            end: 280,
                                                            as_str(): "5",
                                                        },
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                6,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 282,
                                                            end: 283,
                                                            as_str(): "6",
                                                        },
                                                    },
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
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 285,
                                                            end: 286,
                                                            as_str(): "7",
                                                        },
                                                    },
                                                    TyExpression {
                                                        expression: Literal(
                                                            U64(
                                                                8,
                                                            ),
                                                        ),
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 288,
                                                            end: 289,
                                                            as_str(): "8",
                                                        },
                                                    },
                                                ],
                                            },
                                            return_type: TypeId(
                                                7372,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 278,
                                                end: 290,
                                                as_str(): "[5, 6, 7, 8]",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7375,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 263,
                                    end: 291,
                                    as_str(): "[[1, 2, 3, 4], [5, 6, 7, 8]]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7375,
                            ),
                            type_ascription: TypeId(
                                7350,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 239,
                                    end: 260,
                                    as_str(): "[[u64;\n    4];\n    2]",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 232,
                    end: 292,
                    as_str(): "let e: [[u64;\n    4];\n    2] = [[1, 2, 3, 4], [5, 6, 7, 8]];",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 337,
                                    end: 338,
                                    as_str(): "g",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Array {
                                    contents: [
                                        TyExpression {
                                            expression: StructExpression {
                                                struct_name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 16,
                                                        end: 17,
                                                        as_str(): "S",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                fields: [
                                                    TyStructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 354,
                                                                end: 357,
                                                                as_str(): "foo",
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 359,
                                                                end: 361,
                                                                as_str(): "10",
                                                            },
                                                        },
                                                    },
                                                    TyStructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 363,
                                                                end: 366,
                                                                as_str(): "bar",
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 368,
                                                                end: 370,
                                                                as_str(): "20",
                                                            },
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 342,
                                                    end: 343,
                                                    as_str(): "S",
                                                },
                                            },
                                            return_type: TypeId(
                                                7268,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 342,
                                                end: 376,
                                                as_str(): "S {\n        foo: 10, bar: 20\n    }",
                                            },
                                        },
                                        TyExpression {
                                            expression: StructExpression {
                                                struct_name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 16,
                                                        end: 17,
                                                        as_str(): "S",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                fields: [
                                                    TyStructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 394,
                                                                end: 397,
                                                                as_str(): "foo",
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 399,
                                                                end: 400,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                    },
                                                    TyStructExpressionField {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 402,
                                                                end: 405,
                                                                as_str(): "bar",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        value: TyExpression {
                                                            expression: Literal(
                                                                U64(
                                                                    2,
                                                                ),
                                                            ),
                                                            return_type: TypeId(
                                                                21,
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 407,
                                                                end: 408,
                                                                as_str(): "2",
                                                            },
                                                        },
                                                    },
                                                ],
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 382,
                                                    end: 383,
                                                    as_str(): "S",
                                                },
                                            },
                                            return_type: TypeId(
                                                7268,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14c154030,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                ),
                                                start: 382,
                                                end: 414,
                                                as_str(): "S {\n        foo: 1, bar: 2\n    }",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    7390,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 341,
                                    end: 420,
                                    as_str(): "[S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7390,
                            ),
                            type_ascription: TypeId(
                                7376,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 333,
                    end: 421,
                    as_str(): "let g = [S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ];",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 430,
                                    end: 431,
                                    as_str(): "h",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: ArrayIndex {
                                    prefix: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 434,
                                                        end: 435,
                                                        as_str(): "i",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                is_absolute: false,
                                            },
                                            contract_call_params: {},
                                            arguments: [],
                                            function_decl_id: DeclId(
                                                550,
                                                Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 560,
                                                    end: 599,
                                                    as_str(): "fn i() -> [u64;\n4] {\n    [0, 1, 2, 3]\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 434,
                                                        end: 435,
                                                        as_str(): "i",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            7394,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 434,
                                            end: 437,
                                            as_str(): "i()",
                                        },
                                    },
                                    index: TyExpression {
                                        expression: Literal(
                                            U64(
                                                2,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            7395,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 438,
                                            end: 439,
                                            as_str(): "2",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 434,
                                    end: 440,
                                    as_str(): "i()[2]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7391,
                            ),
                            type_ascription: TypeId(
                                7391,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 426,
                    end: 441,
                    as_str(): "let h = i()[2];",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
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
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 452,
                                                                            end: 454,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 452,
                                                                            end: 454,
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
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 452,
                                                                        end: 454,
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
                                                                            src (ptr): 0x00007fb1451bedb0,
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
                                                                        expression: ArrayIndex {
                                                                            prefix: TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 139,
                                                                                            end: 140,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 447,
                                                                                        end: 448,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7399,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 447,
                                                                                    end: 448,
                                                                                    as_str(): "b",
                                                                                },
                                                                            },
                                                                            index: TyExpression {
                                                                                expression: Literal(
                                                                                    U64(
                                                                                        0,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    7400,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 449,
                                                                                    end: 450,
                                                                                    as_str(): "0",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 447,
                                                                            end: 451,
                                                                            as_str(): "b[0]",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1451bedb0,
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
                                                                        expression: ArrayIndex {
                                                                            prefix: TyExpression {
                                                                                expression: VariableExpression {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 139,
                                                                                            end: 140,
                                                                                            as_str(): "b",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 455,
                                                                                        end: 456,
                                                                                        as_str(): "b",
                                                                                    },
                                                                                    mutability: Immutable,
                                                                                },
                                                                                return_type: TypeId(
                                                                                    7404,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 455,
                                                                                    end: 456,
                                                                                    as_str(): "b",
                                                                                },
                                                                            },
                                                                            index: TyExpression {
                                                                                expression: Literal(
                                                                                    U64(
                                                                                        9,
                                                                                    ),
                                                                                ),
                                                                                return_type: TypeId(
                                                                                    7405,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 457,
                                                                                    end: 458,
                                                                                    as_str(): "9",
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 455,
                                                                            end: 459,
                                                                            as_str(): "b[9]",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                551,
                                                                Span {
                                                                    src (ptr): 0x00007fb1451bedb0,
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
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 452,
                                                                        end: 454,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 447,
                                                            end: 459,
                                                            as_str(): "b[0] == b[9]",
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
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 481,
                                                                            end: 483,
                                                                            as_str(): "==",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "ops",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 481,
                                                                            end: 483,
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
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 481,
                                                                        end: 483,
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
                                                                            src (ptr): 0x00007fb1451bedb0,
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
                                                                                prefixes: [
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "core",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 471,
                                                                                            end: 472,
                                                                                            as_str(): "+",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "ops",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 471,
                                                                                            end: 472,
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
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 471,
                                                                                        end: 472,
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
                                                                                            src (ptr): 0x00007fb1451bedb0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 124,
                                                                                            end: 128,
                                                                                            as_str(): "self",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: ArrayIndex {
                                                                                            prefix: TyExpression {
                                                                                                expression: ArrayIndex {
                                                                                                    prefix: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 236,
                                                                                                                    end: 237,
                                                                                                                    as_str(): "e",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 463,
                                                                                                                end: 464,
                                                                                                                as_str(): "e",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            7413,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 463,
                                                                                                            end: 464,
                                                                                                            as_str(): "e",
                                                                                                        },
                                                                                                    },
                                                                                                    index: TyExpression {
                                                                                                        expression: Literal(
                                                                                                            U64(
                                                                                                                0,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            7414,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 465,
                                                                                                            end: 466,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7415,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 463,
                                                                                                    end: 467,
                                                                                                    as_str(): "e[0]",
                                                                                                },
                                                                                            },
                                                                                            index: TyExpression {
                                                                                                expression: Literal(
                                                                                                    U64(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    7416,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 468,
                                                                                                    end: 469,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 463,
                                                                                            end: 470,
                                                                                            as_str(): "e[0][1]",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                                (
                                                                                    BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1451bedb0,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                            ),
                                                                                            start: 130,
                                                                                            end: 135,
                                                                                            as_str(): "other",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    TyExpression {
                                                                                        expression: ArrayIndex {
                                                                                            prefix: TyExpression {
                                                                                                expression: ArrayIndex {
                                                                                                    prefix: TyExpression {
                                                                                                        expression: VariableExpression {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: None,
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 236,
                                                                                                                    end: 237,
                                                                                                                    as_str(): "e",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                                ),
                                                                                                                start: 473,
                                                                                                                end: 474,
                                                                                                                as_str(): "e",
                                                                                                            },
                                                                                                            mutability: Immutable,
                                                                                                        },
                                                                                                        return_type: TypeId(
                                                                                                            7423,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 473,
                                                                                                            end: 474,
                                                                                                            as_str(): "e",
                                                                                                        },
                                                                                                    },
                                                                                                    index: TyExpression {
                                                                                                        expression: Literal(
                                                                                                            U64(
                                                                                                                1,
                                                                                                            ),
                                                                                                        ),
                                                                                                        return_type: TypeId(
                                                                                                            7424,
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 475,
                                                                                                            end: 476,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7425,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 473,
                                                                                                    end: 477,
                                                                                                    as_str(): "e[1]",
                                                                                                },
                                                                                            },
                                                                                            index: TyExpression {
                                                                                                expression: Literal(
                                                                                                    U64(
                                                                                                        2,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    7426,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 478,
                                                                                                    end: 479,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 473,
                                                                                            end: 480,
                                                                                            as_str(): "e[1][2]",
                                                                                        },
                                                                                    },
                                                                                ),
                                                                            ],
                                                                            function_decl_id: DeclId(
                                                                                552,
                                                                                Span {
                                                                                    src (ptr): 0x00007fb1451bedb0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 117,
                                                                                    end: 185,
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
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 471,
                                                                                        end: 472,
                                                                                        as_str(): "+",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        },
                                                                        return_type: TypeId(
                                                                            21,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 463,
                                                                            end: 480,
                                                                            as_str(): "e[0][1] + e[1][2]",
                                                                        },
                                                                    },
                                                                ),
                                                                (
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1451bedb0,
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
                                                                            src (ptr): 0x00007fb14c154030,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                            ),
                                                                            start: 484,
                                                                            end: 485,
                                                                            as_str(): "9",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                553,
                                                                Span {
                                                                    src (ptr): 0x00007fb1451bedb0,
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
                                                                        src (ptr): 0x00007fb14c154030,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                        ),
                                                                        start: 481,
                                                                        end: 483,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            71,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 463,
                                                            end: 485,
                                                            as_str(): "e[0][1] + e[1][2] == 9",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 447,
                                                    end: 485,
                                                    as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9",
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
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 509,
                                                                    end: 511,
                                                                    as_str(): "==",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            BaseIdent {
                                                                name_override_opt: Some(
                                                                    "ops",
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 509,
                                                                    end: 511,
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 509,
                                                                end: 511,
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
                                                                    src (ptr): 0x00007fb1451bedb0,
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
                                                                        prefixes: [
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "core",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 498,
                                                                                    end: 499,
                                                                                    as_str(): "+",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            BaseIdent {
                                                                                name_override_opt: Some(
                                                                                    "ops",
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 498,
                                                                                    end: 499,
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
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 498,
                                                                                end: 499,
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
                                                                                    src (ptr): 0x00007fb1451bedb0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 124,
                                                                                    end: 128,
                                                                                    as_str(): "self",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: StructFieldAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: ArrayIndex {
                                                                                            prefix: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 337,
                                                                                                            end: 338,
                                                                                                            as_str(): "g",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 489,
                                                                                                        end: 490,
                                                                                                        as_str(): "g",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7434,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 489,
                                                                                                    end: 490,
                                                                                                    as_str(): "g",
                                                                                                },
                                                                                            },
                                                                                            index: TyExpression {
                                                                                                expression: Literal(
                                                                                                    U64(
                                                                                                        0,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    7435,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 491,
                                                                                                    end: 492,
                                                                                                    as_str(): "0",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7268,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 489,
                                                                                            end: 493,
                                                                                            as_str(): "g[0]",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: TyStructField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 24,
                                                                                                end: 27,
                                                                                                as_str(): "foo",
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
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 24,
                                                                                            end: 32,
                                                                                            as_str(): "foo: u64",
                                                                                        },
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 29,
                                                                                            end: 32,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    field_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 494,
                                                                                        end: 497,
                                                                                        as_str(): "foo",
                                                                                    },
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7268,
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 489,
                                                                                    end: 497,
                                                                                    as_str(): "g[0].foo",
                                                                                },
                                                                            },
                                                                        ),
                                                                        (
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1451bedb0,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                                    ),
                                                                                    start: 130,
                                                                                    end: 135,
                                                                                    as_str(): "other",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            TyExpression {
                                                                                expression: StructFieldAccess {
                                                                                    prefix: TyExpression {
                                                                                        expression: ArrayIndex {
                                                                                            prefix: TyExpression {
                                                                                                expression: VariableExpression {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                            ),
                                                                                                            start: 337,
                                                                                                            end: 338,
                                                                                                            as_str(): "g",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                        ),
                                                                                                        start: 500,
                                                                                                        end: 501,
                                                                                                        as_str(): "g",
                                                                                                    },
                                                                                                    mutability: Immutable,
                                                                                                },
                                                                                                return_type: TypeId(
                                                                                                    7440,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 500,
                                                                                                    end: 501,
                                                                                                    as_str(): "g",
                                                                                                },
                                                                                            },
                                                                                            index: TyExpression {
                                                                                                expression: Literal(
                                                                                                    U64(
                                                                                                        1,
                                                                                                    ),
                                                                                                ),
                                                                                                return_type: TypeId(
                                                                                                    7441,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                    ),
                                                                                                    start: 502,
                                                                                                    end: 503,
                                                                                                    as_str(): "1",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                        return_type: TypeId(
                                                                                            7268,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 500,
                                                                                            end: 504,
                                                                                            as_str(): "g[1]",
                                                                                        },
                                                                                    },
                                                                                    field_to_access: TyStructField {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c154030,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                                ),
                                                                                                start: 38,
                                                                                                end: 41,
                                                                                                as_str(): "bar",
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
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 38,
                                                                                            end: 46,
                                                                                            as_str(): "bar: u64",
                                                                                        },
                                                                                        type_span: Span {
                                                                                            src (ptr): 0x00007fb14c154030,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                            ),
                                                                                            start: 43,
                                                                                            end: 46,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                        attributes: {},
                                                                                    },
                                                                                    field_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fb14c154030,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                        ),
                                                                                        start: 505,
                                                                                        end: 508,
                                                                                        as_str(): "bar",
                                                                                    },
                                                                                    resolved_type_of_parent: TypeId(
                                                                                        7268,
                                                                                    ),
                                                                                },
                                                                                return_type: TypeId(
                                                                                    21,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c154030,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                    ),
                                                                                    start: 500,
                                                                                    end: 508,
                                                                                    as_str(): "g[1].bar",
                                                                                },
                                                                            },
                                                                        ),
                                                                    ],
                                                                    function_decl_id: DeclId(
                                                                        554,
                                                                        Span {
                                                                            src (ptr): 0x00007fb1451bedb0,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                                            ),
                                                                            start: 117,
                                                                            end: 185,
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
                                                                                src (ptr): 0x00007fb14c154030,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                                ),
                                                                                start: 498,
                                                                                end: 499,
                                                                                as_str(): "+",
                                                                            },
                                                                        },
                                                                    ),
                                                                },
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 489,
                                                                    end: 508,
                                                                    as_str(): "g[0].foo + g[1].bar",
                                                                },
                                                            },
                                                        ),
                                                        (
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1451bedb0,
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
                                                                        12,
                                                                    ),
                                                                ),
                                                                return_type: TypeId(
                                                                    21,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 512,
                                                                    end: 514,
                                                                    as_str(): "12",
                                                                },
                                                            },
                                                        ),
                                                    ],
                                                    function_decl_id: DeclId(
                                                        555,
                                                        Span {
                                                            src (ptr): 0x00007fb1451bedb0,
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
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 509,
                                                                end: 511,
                                                                as_str(): "==",
                                                            },
                                                        },
                                                    ),
                                                },
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 489,
                                                    end: 514,
                                                    as_str(): "g[0].foo + g[1].bar == 12",
                                                },
                                            },
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 447,
                                            end: 514,
                                            as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12",
                                        },
                                    },
                                    rhs: TyExpression {
                                        expression: FunctionApplication {
                                            call_path: CallPath {
                                                prefixes: [],
                                                suffix: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 518,
                                                        end: 519,
                                                        as_str(): "j",
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
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 606,
                                                            end: 613,
                                                            as_str(): "ary_arg",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14c154030,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                    ),
                                                                    start: 337,
                                                                    end: 338,
                                                                    as_str(): "g",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fb14c154030,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                                ),
                                                                start: 520,
                                                                end: 521,
                                                                as_str(): "g",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            7445,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c154030,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                            ),
                                                            start: 520,
                                                            end: 521,
                                                            as_str(): "g",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                557,
                                                Span {
                                                    src (ptr): 0x00007fb14c154030,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                    ),
                                                    start: 601,
                                                    end: 676,
                                                    as_str(): "fn j(ary_arg: [S;\n2]) -> bool {\n    ary_arg[0].foo + ary_arg[1].bar == 12\n}",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb14c154030,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                                        ),
                                                        start: 518,
                                                        end: 519,
                                                        as_str(): "j",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14c154030,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                            ),
                                            start: 518,
                                            end: 522,
                                            as_str(): "j(g)",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 447,
                                    end: 522,
                                    as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g)",
                                },
                            },
                            rhs: TyExpression {
                                expression: Literal(
                                    Boolean(
                                        true,
                                    ),
                                ),
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14c154030,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                                    ),
                                    start: 552,
                                    end: 556,
                                    as_str(): "true",
                                },
                            },
                        },
                        return_type: TypeId(
                            71,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14c154030,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                            ),
                            start: 447,
                            end: 556,
                            as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14c154030,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
                    ),
                    start: 447,
                    end: 556,
                    as_str(): "b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14c154030,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
        ),
        start: 51,
        end: 558,
        as_str(): "fn main() -> bool {\n    let a: [bool;\n    5] = [true, true, true, false, true];\n    let b: [u32;\n    10] = [3;\n    10];\n    let c = [0x01, 0x02, 0x03];\n    let d = [0;\n    10];\n    let e: [[u64;\n    4];\n    2] = [[1, 2, 3, 4], [5, 6, 7, 8]];\n    //let f: [u64; 1 + 1] = [0, 0];\n    let g = [S {\n        foo: 10, bar: 20\n    },\n    S {\n        foo: 1, bar: 2\n    }\n    ];\n    let h = i()[2];\n\n    b[0] == b[9] && e[0][1] + e[1][2] == 9 && g[0].foo + g[1].bar == 12 && j(g) && /* a.len() == 5 && */\n    true\n}",
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
        src (ptr): 0x00007fb14c154030,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZzDMu4/array_basics/src/main.sw",
        ),
        start: 64,
        end: 68,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

