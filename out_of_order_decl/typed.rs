TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 682,
            end: 692,
            as_str(): "FuelStruct",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 699,
                    end: 700,
                    as_str(): "a",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 699,
                end: 706,
                as_str(): "a: bool",
            },
            type_span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 702,
                end: 706,
                as_str(): "bool",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 712,
                    end: 713,
                    as_str(): "b",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                71,
            ),
            initial_type_id: TypeId(
                71,
            ),
            span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 712,
                end: 719,
                as_str(): "b: bool",
            },
            type_span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 715,
                end: 719,
                as_str(): "bool",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 675,
        end: 722,
        as_str(): "struct FuelStruct {\n    a: bool,\n    b: bool,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 768,
            end: 774,
            as_str(): "AnEnum",
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
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 781,
                    end: 788,
                    as_str(): "Variant",
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
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 790,
                end: 792,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 781,
                end: 792,
                as_str(): "Variant: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 763,
        end: 795,
        as_str(): "enum AnEnum {\n    Variant: (),\n}",
    },
    visibility: Private,
}
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 576,
            end: 587,
            as_str(): "FuelWrapper",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 594,
                    end: 595,
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
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 594,
                end: 607,
                as_str(): "a: FuelStruct",
            },
            type_span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 597,
                end: 607,
                as_str(): "FuelStruct",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 613,
                    end: 614,
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
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 613,
                end: 622,
                as_str(): "b: AnEnum",
            },
            type_span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 616,
                end: 622,
                as_str(): "AnEnum",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 569,
        end: 625,
        as_str(): "struct FuelWrapper {\n    a: FuelStruct,\n    b: AnEnum,\n}",
    },
    attributes: {},
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 632,
            end: 643,
            as_str(): "WrapperEnum",
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
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 650,
                    end: 657,
                    as_str(): "Variant",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7258,
            ),
            initial_type_id: TypeId(
                7257,
            ),
            type_span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 659,
                end: 670,
                as_str(): "FuelWrapper",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 650,
                end: 670,
                as_str(): "Variant: FuelWrapper",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 627,
        end: 673,
        as_str(): "enum WrapperEnum {\n    Variant: FuelWrapper,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 727,
            end: 742,
            as_str(): "the_number_five",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                5,
                            ),
                        ),
                        return_type: TypeId(
                            7260,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 758,
                            end: 759,
                            as_str(): "5",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 758,
                    end: 759,
                    as_str(): "5",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 724,
        end: 761,
        as_str(): "fn the_number_five() -> u64 {\n    5\n}",
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
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 748,
        end: 751,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 934,
            end: 938,
            as_str(): "void",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 931,
        end: 944,
        as_str(): "fn void() {\n}",
    },
    attributes: {},
    return_type: TypeId(
        7263,
    ),
    initial_return_type: TypeId(
        7262,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 931,
        end: 940,
        as_str(): "fn void()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 93,
            end: 97,
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
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 118,
                                    end: 119,
                                    as_str(): "a",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        42,
                                    ),
                                ),
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 122,
                                    end: 124,
                                    as_str(): "42",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                7266,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 114,
                    end: 125,
                    as_str(): "let a = 42;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 157,
                                    end: 158,
                                    as_str(): "x",
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
                                                src (ptr): 0x00007fe06cba43c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                ),
                                                start: 161,
                                                end: 176,
                                                as_str(): "the_number_five",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        551,
                                        Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 724,
                                            end: 761,
                                            as_str(): "fn the_number_five() -> u64 {\n    5\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe06cba43c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                ),
                                                start: 161,
                                                end: 176,
                                                as_str(): "the_number_five",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 161,
                                    end: 178,
                                    as_str(): "the_number_five()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7268,
                            ),
                            type_ascription: TypeId(
                                7268,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 153,
                    end: 179,
                    as_str(): "let x = the_number_five();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 213,
                                    end: 214,
                                    as_str(): "z",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe06cba43c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                ),
                                                start: 768,
                                                end: 774,
                                                as_str(): "AnEnum",
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
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 781,
                                                        end: 788,
                                                        as_str(): "Variant",
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
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 790,
                                                    end: 792,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 781,
                                                    end: 792,
                                                    as_str(): "Variant: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 763,
                                            end: 795,
                                            as_str(): "enum AnEnum {\n    Variant: (),\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 781,
                                            end: 788,
                                            as_str(): "Variant",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 217,
                                        end: 223,
                                        as_str(): "AnEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 225,
                                        end: 232,
                                        as_str(): "Variant",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 225,
                                            end: 232,
                                            as_str(): "Variant",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7256,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 225,
                                    end: 232,
                                    as_str(): "Variant",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7256,
                            ),
                            type_ascription: TypeId(
                                7269,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 209,
                    end: 233,
                    as_str(): "let z = AnEnum::Variant;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 269,
                                    end: 270,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 682,
                                            end: 692,
                                            as_str(): "FuelStruct",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 294,
                                                    end: 295,
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
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 297,
                                                    end: 301,
                                                    as_str(): "true",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 311,
                                                    end: 312,
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
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 314,
                                                    end: 319,
                                                    as_str(): "false",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 273,
                                        end: 283,
                                        as_str(): "FuelStruct",
                                    },
                                },
                                return_type: TypeId(
                                    7254,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 273,
                                    end: 326,
                                    as_str(): "FuelStruct {\n        a: true,\n        b: false,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7254,
                            ),
                            type_ascription: TypeId(
                                7270,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 265,
                    end: 327,
                    as_str(): "let y = FuelStruct {\n        a: true,\n        b: false,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 395,
                                    end: 396,
                                    as_str(): "u",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: StructExpression {
                                    struct_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 576,
                                            end: 587,
                                            as_str(): "FuelWrapper",
                                        },
                                        is_raw_ident: false,
                                    },
                                    fields: [
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 421,
                                                    end: 422,
                                                    as_str(): "a",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 269,
                                                            end: 270,
                                                            as_str(): "y",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 424,
                                                        end: 425,
                                                        as_str(): "y",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7254,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 424,
                                                    end: 425,
                                                    as_str(): "y",
                                                },
                                            },
                                        },
                                        TyStructExpressionField {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 435,
                                                    end: 436,
                                                    as_str(): "b",
                                                },
                                                is_raw_ident: false,
                                            },
                                            value: TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 213,
                                                            end: 214,
                                                            as_str(): "z",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 438,
                                                        end: 439,
                                                        as_str(): "z",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7256,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 438,
                                                    end: 439,
                                                    as_str(): "z",
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 399,
                                        end: 410,
                                        as_str(): "FuelWrapper",
                                    },
                                },
                                return_type: TypeId(
                                    7258,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 399,
                                    end: 446,
                                    as_str(): "FuelWrapper {\n        a: y,\n        b: z,\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7258,
                            ),
                            type_ascription: TypeId(
                                7274,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 391,
                    end: 447,
                    as_str(): "let u = FuelWrapper {\n        a: y,\n        b: z,\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 457,
                                    end: 458,
                                    as_str(): "v",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe06cba43c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                ),
                                                start: 632,
                                                end: 643,
                                                as_str(): "WrapperEnum",
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
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 650,
                                                        end: 657,
                                                        as_str(): "Variant",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7258,
                                                ),
                                                initial_type_id: TypeId(
                                                    7257,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 659,
                                                    end: 670,
                                                    as_str(): "FuelWrapper",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 650,
                                                    end: 670,
                                                    as_str(): "Variant: FuelWrapper",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 627,
                                            end: 673,
                                            as_str(): "enum WrapperEnum {\n    Variant: FuelWrapper,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 650,
                                            end: 657,
                                            as_str(): "Variant",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe06cba43c0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                        ),
                                                        start: 395,
                                                        end: 396,
                                                        as_str(): "u",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 482,
                                                    end: 483,
                                                    as_str(): "u",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                7258,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe06cba43c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                ),
                                                start: 482,
                                                end: 483,
                                                as_str(): "u",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 461,
                                        end: 472,
                                        as_str(): "WrapperEnum",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 474,
                                        end: 481,
                                        as_str(): "Variant",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe06cba43c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                            ),
                                            start: 461,
                                            end: 481,
                                            as_str(): "WrapperEnum::Variant",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7279,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 474,
                                    end: 481,
                                    as_str(): "Variant",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7279,
                            ),
                            type_ascription: TypeId(
                                7278,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 453,
                    end: 485,
                    as_str(): "let v = WrapperEnum::Variant(u);",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 524,
                                                    end: 531,
                                                    as_str(): "Variant",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 524,
                                                    end: 531,
                                                    as_str(): "Variant",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: Some(
                                                "eq",
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fe06cba43c0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                ),
                                                start: 524,
                                                end: 531,
                                                as_str(): "Variant",
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
                                                    src (ptr): 0x00007fe0708306e0,
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
                                                expression: EnumTag {
                                                    exp: TyExpression {
                                                        expression: EnumInstantiation {
                                                            enum_decl: TyEnumDeclaration {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 768,
                                                                        end: 774,
                                                                        as_str(): "AnEnum",
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
                                                                                src (ptr): 0x00007fe06cba43c0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                                ),
                                                                                start: 781,
                                                                                end: 788,
                                                                                as_str(): "Variant",
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
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 790,
                                                                            end: 792,
                                                                            as_str(): "()",
                                                                        },
                                                                        tag: 0,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe06cba43c0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                            ),
                                                                            start: 781,
                                                                            end: 792,
                                                                            as_str(): "Variant: ()",
                                                                        },
                                                                        attributes: {},
                                                                    },
                                                                ],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 763,
                                                                    end: 795,
                                                                    as_str(): "enum AnEnum {\n    Variant: (),\n}",
                                                                },
                                                                visibility: Private,
                                                            },
                                                            variant_name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 781,
                                                                    end: 788,
                                                                    as_str(): "Variant",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            tag: 0,
                                                            contents: None,
                                                            enum_instantiation_span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 516,
                                                                end: 522,
                                                                as_str(): "AnEnum",
                                                            },
                                                            variant_instantiation_span: Span {
                                                                src (ptr): 0x00007fe06cba43c0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                ),
                                                                start: 524,
                                                                end: 531,
                                                                as_str(): "Variant",
                                                            },
                                                            type_binding: TypeBinding {
                                                                inner: (),
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 524,
                                                                    end: 531,
                                                                    as_str(): "Variant",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            7256,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 524,
                                                            end: 531,
                                                            as_str(): "Variant",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 524,
                                                    end: 531,
                                                    as_str(): "Variant",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0708306e0,
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
                                                        0,
                                                    ),
                                                ),
                                                return_type: TypeId(
                                                    21,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 524,
                                                    end: 531,
                                                    as_str(): "Variant",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        557,
                                        Span {
                                            src (ptr): 0x00007fe0708306e0,
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
                                    type_binding: None,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 524,
                                    end: 531,
                                    as_str(): "Variant",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: FunctionApplication {
                                                            call_path: CallPath {
                                                                prefixes: [],
                                                                suffix: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 542,
                                                                        end: 546,
                                                                        as_str(): "void",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            contract_call_params: {},
                                                            arguments: [],
                                                            function_decl_id: DeclId(
                                                                556,
                                                                Span {
                                                                    src (ptr): 0x00007fe06cba43c0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                    ),
                                                                    start: 931,
                                                                    end: 944,
                                                                    as_str(): "fn void() {\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe06cba43c0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                                        ),
                                                                        start: 542,
                                                                        end: 546,
                                                                        as_str(): "void",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            7287,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe06cba43c0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                            ),
                                                            start: 542,
                                                            end: 548,
                                                            as_str(): "void()",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe06cba43c0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                                    ),
                                                    start: 542,
                                                    end: 548,
                                                    as_str(): "void()",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7289,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe06cba43c0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                    ),
                                    start: 532,
                                    end: 555,
                                    as_str(): "{\n        void();\n    }",
                                },
                            },
                            else: Some(
                                TyExpression {
                                    expression: CodeBlock(
                                        TyCodeBlock {
                                            contents: [],
                                        },
                                    ),
                                    return_type: TypeId(
                                        7294,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe06cba43c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                                        ),
                                        start: 532,
                                        end: 555,
                                        as_str(): "{\n        void();\n    }",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            7295,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 532,
                            end: 555,
                            as_str(): "{\n        void();\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 491,
                    end: 555,
                    as_str(): "if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }",
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
                            src (ptr): 0x00007fe06cba43c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                            ),
                            start: 561,
                            end: 565,
                            as_str(): "true",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe06cba43c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                    ),
                    start: 561,
                    end: 565,
                    as_str(): "true",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 90,
        end: 567,
        as_str(): "fn main() -> bool {\n    let a = 42;\n\n    // fn before decl\n    let x = the_number_five();\n\n    // enum before decl\n    let z = AnEnum::Variant;\n\n    // struct before decl\n    let y = FuelStruct {\n        a: true,\n        b: false,\n    };\n\n    // struct and enum with complex members, out of order\n    let u = FuelWrapper {\n        a: y,\n        b: z,\n    };\n\n    let v = WrapperEnum::Variant(u);\n\n    if let AnEnum::Variant = AnEnum::Variant {\n        void();\n    }\n\n    true\n}",
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
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 103,
        end: 107,
        as_str(): "bool",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 894,
            end: 903,
            as_str(): "FuelTrait",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    interface_surface: [
        DeclId(
            559,
            Span {
                src (ptr): 0x00007fe06cba43c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
                ),
                start: 913,
                end: 916,
                as_str(): "foo",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe06cba43c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
        ),
        start: 888,
        end: 929,
        as_str(): "trait FuelTrait {\n    fn foo() -> bool;\n}",
    },
}
ImplTrait(
    DeclId(
        563,
        Span {
            src (ptr): 0x00007fe06cba43c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR6nTOpL/out_of_order_decl/src/main.sw",
            ),
            start: 818,
            end: 886,
            as_str(): "impl FuelTrait for u64 {\n    fn foo() -> bool {\n        true\n    }\n}",
        },
    ),
)

