
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb1443204a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
            ),
            start: 40,
            end: 46,
            as_str(): "Result",
        },
        is_raw_ident: false,
    },
    type_parameters: [
        T: TypeId(31633),
        E: TypeId(31634),
    ],
    attributes: {},
    variants: [
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 59,
                    end: 61,
                    as_str(): "Ok",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31633,
            ),
            initial_type_id: TypeId(
                31635,
            ),
            type_span: Span {
                src (ptr): 0x00007fb1443204a0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                ),
                start: 63,
                end: 64,
                as_str(): "T",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb1443204a0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                ),
                start: 59,
                end: 64,
                as_str(): "Ok: T",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 70,
                    end: 73,
                    as_str(): "Err",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31634,
            ),
            initial_type_id: TypeId(
                31636,
            ),
            type_span: Span {
                src (ptr): 0x00007fb1443204a0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                ),
                start: 75,
                end: 76,
                as_str(): "E",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb1443204a0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                ),
                start: 70,
                end: 76,
                as_str(): "Err: E",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb1443204a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
        ),
        start: 35,
        end: 79,
        as_str(): "enum Result<T, E> {\n    Ok: T,\n    Err: E,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb1443204a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
            ),
            start: 84,
            end: 95,
            as_str(): "local_panic",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: IntrinsicFunction(
                            TyIntrinsicFunctionKind {
                                kind: Revert,
                                arguments: [
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
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 121,
                                            end: 123,
                                            as_str(): "42",
                                        },
                                    },
                                ],
                                type_arguments: [],
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 112,
                                    end: 124,
                                    as_str(): "__revert(42)",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31642,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 112,
                            end: 124,
                            as_str(): "__revert(42)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 112,
                    end: 124,
                    as_str(): "__revert(42)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb1443204a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
        ),
        start: 81,
        end: 126,
        as_str(): "fn local_panic<T>() -> T {\n    __revert(42)\n}",
    },
    attributes: {},
    return_type: TypeId(
        31638,
    ),
    initial_return_type: TypeId(
        31639,
    ),
    type_parameters: [
        T: TypeId(31638),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fb1443204a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
        ),
        start: 104,
        end: 105,
        as_str(): "T",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb1443204a0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
            ),
            start: 131,
            end: 135,
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
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 265,
                                    end: 266,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: IfExp {
                                    condition: TyExpression {
                                        expression: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 272,
                                            end: 276,
                                            as_str(): "true",
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
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 279,
                                                                    end: 284,
                                                                    as_str(): "42u64",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 279,
                                                            end: 284,
                                                            as_str(): "42u64",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        return_type: TypeId(
                                            21,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 277,
                                            end: 286,
                                            as_str(): "{ 42u64 }",
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
                                                                    expression: FunctionApplication {
                                                                        call_path: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 294,
                                                                                    end: 300,
                                                                                    as_str(): "revert",
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
                                                                                        src (ptr): 0x00007fb14c3b8e40,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                        ),
                                                                                        start: 582,
                                                                                        end: 586,
                                                                                        as_str(): "code",
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
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 301,
                                                                                        end: 302,
                                                                                        as_str(): "0",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            13317,
                                                                            Span {
                                                                                src (ptr): 0x00007fb14c3b8e40,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                                                                ),
                                                                                start: 568,
                                                                                end: 615,
                                                                                as_str(): "pub fn revert(code: u64) {\n    __revert(code)\n}",
                                                                            },
                                                                        ),
                                                                        self_state_idx: None,
                                                                        selector: None,
                                                                        type_binding: Some(
                                                                            TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 294,
                                                                                    end: 300,
                                                                                    as_str(): "revert",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        31649,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 294,
                                                                        end: 303,
                                                                        as_str(): "revert(0)",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 294,
                                                                end: 303,
                                                                as_str(): "revert(0)",
                                                            },
                                                        },
                                                    ],
                                                },
                                            ),
                                            return_type: TypeId(
                                                31650,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb1443204a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                ),
                                                start: 292,
                                                end: 305,
                                                as_str(): "{ revert(0) }",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 269,
                                    end: 305,
                                    as_str(): "if true { 42u64 } else { revert(0) }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31644,
                            ),
                            type_ascription: TypeId(
                                31644,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 261,
                    end: 306,
                    as_str(): "let x = if true { 42u64 } else { revert(0) };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 315,
                                    end: 316,
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
                                                src (ptr): 0x00007fb1443204a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                ),
                                                start: 324,
                                                end: 335,
                                                as_str(): "local_panic",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        13319,
                                        Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 81,
                                            end: 126,
                                            as_str(): "fn local_panic<T>() -> T {\n    __revert(42)\n}",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [
                                                TypeArgument {
                                                    type_id: TypeId(
                                                        21,
                                                    ),
                                                    initial_type_id: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 338,
                                                        end: 341,
                                                        as_str(): "u64",
                                                    },
                                                },
                                            ],
                                            span: Span {
                                                src (ptr): 0x00007fb1443204a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                ),
                                                start: 324,
                                                end: 342,
                                                as_str(): "local_panic::<u64>",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 324,
                                    end: 344,
                                    as_str(): "local_panic::<u64>()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 318,
                                    end: 321,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 311,
                    end: 345,
                    as_str(): "let x: u64 = local_panic::<u64>();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 354,
                                    end: 355,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
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
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 390,
                                                            end: 392,
                                                            as_str(): "Ok",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 390,
                                                            end: 392,
                                                            as_str(): "Ok",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "eq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb1443204a0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                        ),
                                                        start: 390,
                                                        end: 392,
                                                        as_str(): "Ok",
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
                                                            src (ptr): 0x00007fb14c0ea310,
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
                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                ),
                                                                                start: 1808,
                                                                                end: 1814,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_parameters: [
                                                                            T: TypeId(21),
                                                                            E: TypeId(21),
                                                                        ],
                                                                        attributes: {
                                                                            DocComment: [
                                                                                Attribute {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "doc-comment",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                            ),
                                                                                            start: 1710,
                                                                                            end: 1783,
                                                                                            as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    args: [
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1713,
                                                                                                end: 1783,
                                                                                                as_str(): " `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1710,
                                                                                        end: 1783,
                                                                                        as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                    },
                                                                                },
                                                                                Attribute {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "doc-comment",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                            ),
                                                                                            start: 1784,
                                                                                            end: 1798,
                                                                                            as_str(): "/// ([`Err`]).",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    args: [
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1787,
                                                                                                end: 1798,
                                                                                                as_str(): " ([`Err`]).",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1784,
                                                                                        end: 1798,
                                                                                        as_str(): "/// ([`Err`]).",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        variants: [
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1862,
                                                                                        end: 1864,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7487,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1866,
                                                                                    end: 1867,
                                                                                    as_str(): "T",
                                                                                },
                                                                                tag: 0,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1862,
                                                                                    end: 1867,
                                                                                    as_str(): "Ok: T",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1827,
                                                                                                    end: 1857,
                                                                                                    as_str(): "/// Contains the success value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1830,
                                                                                                        end: 1857,
                                                                                                        as_str(): " Contains the success value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1827,
                                                                                                end: 1857,
                                                                                                as_str(): "/// Contains the success value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1906,
                                                                                        end: 1909,
                                                                                        as_str(): "Err",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7488,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1911,
                                                                                    end: 1912,
                                                                                    as_str(): "E",
                                                                                },
                                                                                tag: 1,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1906,
                                                                                    end: 1912,
                                                                                    as_str(): "Err: E",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1873,
                                                                                                    end: 1901,
                                                                                                    as_str(): "/// Contains the error value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1876,
                                                                                                        end: 1901,
                                                                                                        as_str(): " Contains the error value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1873,
                                                                                                end: 1901,
                                                                                                as_str(): "/// Contains the error value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                            ),
                                                                            start: 1799,
                                                                            end: 1915,
                                                                            as_str(): "pub enum Result<T, E> {\n    /// Contains the success value\n    Ok: T,\n    /// Contains the error value\n    Err: E,\n}",
                                                                        },
                                                                        visibility: Public,
                                                                    },
                                                                    variant_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                            ),
                                                                            start: 1862,
                                                                            end: 1864,
                                                                            as_str(): "Ok",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    tag: 0,
                                                                    contents: Some(
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
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 405,
                                                                                end: 406,
                                                                                as_str(): "5",
                                                                            },
                                                                        },
                                                                    ),
                                                                    enum_instantiation_span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 382,
                                                                        end: 388,
                                                                        as_str(): "Result",
                                                                    },
                                                                    variant_instantiation_span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 390,
                                                                        end: 392,
                                                                        as_str(): "Ok",
                                                                    },
                                                                    type_binding: TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7259,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 395,
                                                                                    end: 398,
                                                                                    as_str(): "u64",
                                                                                },
                                                                            },
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7260,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 400,
                                                                                    end: 403,
                                                                                    as_str(): "u64",
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 382,
                                                                            end: 404,
                                                                            as_str(): "Result::Ok::<u64, u64>",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    31680,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 390,
                                                                    end: 392,
                                                                    as_str(): "Ok",
                                                                },
                                                            },
                                                        },
                                                        return_type: TypeId(
                                                            21,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 390,
                                                            end: 392,
                                                            as_str(): "Ok",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14c0ea310,
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
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 390,
                                                            end: 392,
                                                            as_str(): "Ok",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13339,
                                                Span {
                                                    src (ptr): 0x00007fb14c0ea310,
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
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 390,
                                            end: 392,
                                            as_str(): "Ok",
                                        },
                                    },
                                    then: TyExpression {
                                        expression: CodeBlock(
                                            TyCodeBlock {
                                                contents: [
                                                    TyAstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                TyVariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 378,
                                                                            as_str(): "ok",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    body: TyExpression {
                                                                        expression: UnsafeDowncast {
                                                                            exp: TyExpression {
                                                                                expression: EnumInstantiation {
                                                                                    enum_decl: TyEnumDeclaration {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1808,
                                                                                                end: 1814,
                                                                                                as_str(): "Result",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        type_parameters: [
                                                                                            T: TypeId(21),
                                                                                            E: TypeId(21),
                                                                                        ],
                                                                                        attributes: {
                                                                                            DocComment: [
                                                                                                Attribute {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "doc-comment",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                                            path: Some(
                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                            ),
                                                                                                            start: 1710,
                                                                                                            end: 1783,
                                                                                                            as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    args: [
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                ),
                                                                                                                start: 1713,
                                                                                                                end: 1783,
                                                                                                                as_str(): " `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1710,
                                                                                                        end: 1783,
                                                                                                        as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                                    },
                                                                                                },
                                                                                                Attribute {
                                                                                                    name: BaseIdent {
                                                                                                        name_override_opt: Some(
                                                                                                            "doc-comment",
                                                                                                        ),
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                                            path: Some(
                                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                            ),
                                                                                                            start: 1784,
                                                                                                            end: 1798,
                                                                                                            as_str(): "/// ([`Err`]).",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    args: [
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: None,
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                ),
                                                                                                                start: 1787,
                                                                                                                end: 1798,
                                                                                                                as_str(): " ([`Err`]).",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                    ],
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1784,
                                                                                                        end: 1798,
                                                                                                        as_str(): "/// ([`Err`]).",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                        variants: [
                                                                                            TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1862,
                                                                                                        end: 1864,
                                                                                                        as_str(): "Ok",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7487,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1866,
                                                                                                    end: 1867,
                                                                                                    as_str(): "T",
                                                                                                },
                                                                                                tag: 0,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1862,
                                                                                                    end: 1867,
                                                                                                    as_str(): "Ok: T",
                                                                                                },
                                                                                                attributes: {
                                                                                                    DocComment: [
                                                                                                        Attribute {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "doc-comment",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                    ),
                                                                                                                    start: 1827,
                                                                                                                    end: 1857,
                                                                                                                    as_str(): "/// Contains the success value",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            args: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                        ),
                                                                                                                        start: 1830,
                                                                                                                        end: 1857,
                                                                                                                        as_str(): " Contains the success value",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                ),
                                                                                                                start: 1827,
                                                                                                                end: 1857,
                                                                                                                as_str(): "/// Contains the success value",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            },
                                                                                            TyEnumVariant {
                                                                                                name: BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1906,
                                                                                                        end: 1909,
                                                                                                        as_str(): "Err",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                type_id: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7488,
                                                                                                ),
                                                                                                type_span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1911,
                                                                                                    end: 1912,
                                                                                                    as_str(): "E",
                                                                                                },
                                                                                                tag: 1,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1906,
                                                                                                    end: 1912,
                                                                                                    as_str(): "Err: E",
                                                                                                },
                                                                                                attributes: {
                                                                                                    DocComment: [
                                                                                                        Attribute {
                                                                                                            name: BaseIdent {
                                                                                                                name_override_opt: Some(
                                                                                                                    "doc-comment",
                                                                                                                ),
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                                    path: Some(
                                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                    ),
                                                                                                                    start: 1873,
                                                                                                                    end: 1901,
                                                                                                                    as_str(): "/// Contains the error value",
                                                                                                                },
                                                                                                                is_raw_ident: false,
                                                                                                            },
                                                                                                            args: [
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                                        path: Some(
                                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                        ),
                                                                                                                        start: 1876,
                                                                                                                        end: 1901,
                                                                                                                        as_str(): " Contains the error value",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ],
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                                path: Some(
                                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                                ),
                                                                                                                start: 1873,
                                                                                                                end: 1901,
                                                                                                                as_str(): "/// Contains the error value",
                                                                                                            },
                                                                                                        },
                                                                                                    ],
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                            ),
                                                                                            start: 1799,
                                                                                            end: 1915,
                                                                                            as_str(): "pub enum Result<T, E> {\n    /// Contains the success value\n    Ok: T,\n    /// Contains the error value\n    Err: E,\n}",
                                                                                        },
                                                                                        visibility: Public,
                                                                                    },
                                                                                    variant_name: BaseIdent {
                                                                                        name_override_opt: None,
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                            ),
                                                                                            start: 1862,
                                                                                            end: 1864,
                                                                                            as_str(): "Ok",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    tag: 0,
                                                                                    contents: Some(
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
                                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                ),
                                                                                                start: 405,
                                                                                                end: 406,
                                                                                                as_str(): "5",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    enum_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 382,
                                                                                        end: 388,
                                                                                        as_str(): "Result",
                                                                                    },
                                                                                    variant_instantiation_span: Span {
                                                                                        src (ptr): 0x00007fb1443204a0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                        ),
                                                                                        start: 390,
                                                                                        end: 392,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    type_binding: TypeBinding {
                                                                                        inner: (),
                                                                                        type_arguments: [
                                                                                            TypeArgument {
                                                                                                type_id: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7259,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                    ),
                                                                                                    start: 395,
                                                                                                    end: 398,
                                                                                                    as_str(): "u64",
                                                                                                },
                                                                                            },
                                                                                            TypeArgument {
                                                                                                type_id: TypeId(
                                                                                                    21,
                                                                                                ),
                                                                                                initial_type_id: TypeId(
                                                                                                    7260,
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                                    ),
                                                                                                    start: 400,
                                                                                                    end: 403,
                                                                                                    as_str(): "u64",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 382,
                                                                                            end: 404,
                                                                                            as_str(): "Result::Ok::<u64, u64>",
                                                                                        },
                                                                                    },
                                                                                },
                                                                                return_type: TypeId(
                                                                                    31680,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 390,
                                                                                    end: 392,
                                                                                    as_str(): "Ok",
                                                                                },
                                                                            },
                                                                            variant: TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1862,
                                                                                        end: 1864,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    31681,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7487,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1866,
                                                                                    end: 1867,
                                                                                    as_str(): "T",
                                                                                },
                                                                                tag: 0,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1862,
                                                                                    end: 1867,
                                                                                    as_str(): "Ok: T",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1827,
                                                                                                    end: 1857,
                                                                                                    as_str(): "/// Contains the success value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1830,
                                                                                                        end: 1857,
                                                                                                        as_str(): " Contains the success value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1827,
                                                                                                end: 1857,
                                                                                                as_str(): "/// Contains the success value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                        },
                                                                        return_type: TypeId(
                                                                            31681,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 365,
                                                                            end: 379,
                                                                            as_str(): "Result::Ok(ok)",
                                                                        },
                                                                    },
                                                                    mutability: Immutable,
                                                                    return_type: TypeId(
                                                                        31681,
                                                                    ),
                                                                    type_ascription: TypeId(
                                                                        31681,
                                                                    ),
                                                                    type_ascription_span: None,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 376,
                                                            end: 378,
                                                            as_str(): "ok",
                                                        },
                                                    },
                                                    TyAstNode {
                                                        content: ImplicitReturnExpression(
                                                            TyExpression {
                                                                expression: VariableExpression {
                                                                    name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 376,
                                                                            end: 378,
                                                                            as_str(): "ok",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 418,
                                                                        end: 420,
                                                                        as_str(): "ok",
                                                                    },
                                                                    mutability: Immutable,
                                                                },
                                                                return_type: TypeId(
                                                                    31681,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 418,
                                                                    end: 420,
                                                                    as_str(): "ok",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 418,
                                                            end: 420,
                                                            as_str(): "ok",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        return_type: TypeId(
                                            31681,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 408,
                                            end: 426,
                                            as_str(): "{\n        ok\n    }",
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
                                                                    expression: FunctionApplication {
                                                                        call_path: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 442,
                                                                                    end: 453,
                                                                                    as_str(): "local_panic",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: false,
                                                                        },
                                                                        contract_call_params: {},
                                                                        arguments: [],
                                                                        function_decl_id: DeclId(
                                                                            13338,
                                                                            Span {
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 81,
                                                                                end: 126,
                                                                                as_str(): "fn local_panic<T>() -> T {\n    __revert(42)\n}",
                                                                            },
                                                                        ),
                                                                        self_state_idx: None,
                                                                        selector: None,
                                                                        type_binding: Some(
                                                                            TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [
                                                                                    TypeArgument {
                                                                                        type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        initial_type_id: TypeId(
                                                                                            21,
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1443204a0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                            ),
                                                                                            start: 456,
                                                                                            end: 459,
                                                                                            as_str(): "u64",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 442,
                                                                                    end: 460,
                                                                                    as_str(): "local_panic::<u64>",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 442,
                                                                        end: 462,
                                                                        as_str(): "local_panic::<u64>()",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 442,
                                                                end: 462,
                                                                as_str(): "local_panic::<u64>()",
                                                            },
                                                        },
                                                    ],
                                                },
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb1443204a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                ),
                                                start: 432,
                                                end: 468,
                                                as_str(): "{\n        local_panic::<u64>()\n    }",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31681,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 408,
                                    end: 426,
                                    as_str(): "{\n        ok\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31651,
                            ),
                            type_ascription: TypeId(
                                31651,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 350,
                    end: 469,
                    as_str(): "let x = if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    };",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 478,
                                    end: 479,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: IfExp {
                                    condition: TyExpression {
                                        expression: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 485,
                                            end: 489,
                                            as_str(): "true",
                                        },
                                    },
                                    then: TyExpression {
                                        expression: CodeBlock(
                                            TyCodeBlock {
                                                contents: [
                                                    TyAstNode {
                                                        content: ImplicitReturnExpression(
                                                            TyExpression {
                                                                expression: EnumInstantiation {
                                                                    enum_decl: TyEnumDeclaration {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                path: Some(
                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                ),
                                                                                start: 1808,
                                                                                end: 1814,
                                                                                as_str(): "Result",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_parameters: [
                                                                            T: TypeId(21),
                                                                            E: TypeId(33),
                                                                        ],
                                                                        attributes: {
                                                                            DocComment: [
                                                                                Attribute {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "doc-comment",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                            ),
                                                                                            start: 1710,
                                                                                            end: 1783,
                                                                                            as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    args: [
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1713,
                                                                                                end: 1783,
                                                                                                as_str(): " `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1710,
                                                                                        end: 1783,
                                                                                        as_str(): "/// `Result` is a type that represents either success ([`Ok`]) or failure",
                                                                                    },
                                                                                },
                                                                                Attribute {
                                                                                    name: BaseIdent {
                                                                                        name_override_opt: Some(
                                                                                            "doc-comment",
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                                            path: Some(
                                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                            ),
                                                                                            start: 1784,
                                                                                            end: 1798,
                                                                                            as_str(): "/// ([`Err`]).",
                                                                                        },
                                                                                        is_raw_ident: false,
                                                                                    },
                                                                                    args: [
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1787,
                                                                                                end: 1798,
                                                                                                as_str(): " ([`Err`]).",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1784,
                                                                                        end: 1798,
                                                                                        as_str(): "/// ([`Err`]).",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                        variants: [
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1862,
                                                                                        end: 1864,
                                                                                        as_str(): "Ok",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7487,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1866,
                                                                                    end: 1867,
                                                                                    as_str(): "T",
                                                                                },
                                                                                tag: 0,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1862,
                                                                                    end: 1867,
                                                                                    as_str(): "Ok: T",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1827,
                                                                                                    end: 1857,
                                                                                                    as_str(): "/// Contains the success value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1830,
                                                                                                        end: 1857,
                                                                                                        as_str(): " Contains the success value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1827,
                                                                                                end: 1857,
                                                                                                as_str(): "/// Contains the success value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                            TyEnumVariant {
                                                                                name: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                        path: Some(
                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                        ),
                                                                                        start: 1906,
                                                                                        end: 1909,
                                                                                        as_str(): "Err",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                type_id: TypeId(
                                                                                    33,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7488,
                                                                                ),
                                                                                type_span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1911,
                                                                                    end: 1912,
                                                                                    as_str(): "E",
                                                                                },
                                                                                tag: 1,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                    path: Some(
                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                    ),
                                                                                    start: 1906,
                                                                                    end: 1912,
                                                                                    as_str(): "Err: E",
                                                                                },
                                                                                attributes: {
                                                                                    DocComment: [
                                                                                        Attribute {
                                                                                            name: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "doc-comment",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb14c4dc120,
                                                                                                    path: Some(
                                                                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                    ),
                                                                                                    start: 1873,
                                                                                                    end: 1901,
                                                                                                    as_str(): "/// Contains the error value",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            args: [
                                                                                                BaseIdent {
                                                                                                    name_override_opt: None,
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb14c4dc120,
                                                                                                        path: Some(
                                                                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                        ),
                                                                                                        start: 1876,
                                                                                                        end: 1901,
                                                                                                        as_str(): " Contains the error value",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb14c4dc120,
                                                                                                path: Some(
                                                                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                                                ),
                                                                                                start: 1873,
                                                                                                end: 1901,
                                                                                                as_str(): "/// Contains the error value",
                                                                                            },
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                            ),
                                                                            start: 1799,
                                                                            end: 1915,
                                                                            as_str(): "pub enum Result<T, E> {\n    /// Contains the success value\n    Ok: T,\n    /// Contains the error value\n    Err: E,\n}",
                                                                        },
                                                                        visibility: Public,
                                                                    },
                                                                    variant_name: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14c4dc120,
                                                                            path: Some(
                                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/result.sw",
                                                                            ),
                                                                            start: 1906,
                                                                            end: 1909,
                                                                            as_str(): "Err",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    tag: 1,
                                                                    contents: Some(
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
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 524,
                                                                                end: 526,
                                                                                as_str(): "12",
                                                                            },
                                                                        },
                                                                    ),
                                                                    enum_instantiation_span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 500,
                                                                        end: 506,
                                                                        as_str(): "Result",
                                                                    },
                                                                    variant_instantiation_span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 508,
                                                                        end: 511,
                                                                        as_str(): "Err",
                                                                    },
                                                                    type_binding: TypeBinding {
                                                                        inner: (),
                                                                        type_arguments: [
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    21,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7259,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 514,
                                                                                    end: 517,
                                                                                    as_str(): "u64",
                                                                                },
                                                                            },
                                                                            TypeArgument {
                                                                                type_id: TypeId(
                                                                                    33,
                                                                                ),
                                                                                initial_type_id: TypeId(
                                                                                    7260,
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1443204a0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                    ),
                                                                                    start: 519,
                                                                                    end: 522,
                                                                                    as_str(): "u32",
                                                                                },
                                                                            },
                                                                        ],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1443204a0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                            ),
                                                                            start: 500,
                                                                            end: 523,
                                                                            as_str(): "Result::Err::<u64, u32>",
                                                                        },
                                                                    },
                                                                },
                                                                return_type: TypeId(
                                                                    31718,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1443204a0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                    ),
                                                                    start: 508,
                                                                    end: 511,
                                                                    as_str(): "Err",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1443204a0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                            ),
                                                            start: 500,
                                                            end: 527,
                                                            as_str(): "Result::Err::<u64, u32>(12)",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        return_type: TypeId(
                                            31718,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1443204a0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                            ),
                                            start: 490,
                                            end: 533,
                                            as_str(): "{\n        Result::Err::<u64, u32>(12)\n    }",
                                        },
                                    },
                                    else: Some(
                                        TyExpression {
                                            expression: CodeBlock(
                                                TyCodeBlock {
                                                    contents: [
                                                        TyAstNode {
                                                            content: Expression(
                                                                TyExpression {
                                                                    expression: Return(
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
                                                                                src (ptr): 0x00007fb1443204a0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                                ),
                                                                                start: 556,
                                                                                end: 558,
                                                                                as_str(): "10",
                                                                            },
                                                                        },
                                                                    ),
                                                                    return_type: TypeId(
                                                                        31723,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1443204a0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                        ),
                                                                        start: 549,
                                                                        end: 558,
                                                                        as_str(): "return 10",
                                                                    },
                                                                },
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1443204a0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                                ),
                                                                start: 549,
                                                                end: 558,
                                                                as_str(): "return 10",
                                                            },
                                                        },
                                                    ],
                                                },
                                            ),
                                            return_type: TypeId(
                                                7215,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb1443204a0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                                ),
                                                start: 539,
                                                end: 565,
                                                as_str(): "{\n        return 10;\n    }",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    31718,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 482,
                                    end: 565,
                                    as_str(): "if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    }",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31718,
                            ),
                            type_ascription: TypeId(
                                31689,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 474,
                    end: 566,
                    as_str(): "let x = if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    };",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: Return(
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
                                    src (ptr): 0x00007fb1443204a0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                                    ),
                                    start: 578,
                                    end: 580,
                                    as_str(): "42",
                                },
                            },
                        ),
                        return_type: TypeId(
                            31727,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1443204a0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                            ),
                            start: 571,
                            end: 580,
                            as_str(): "return 42",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb1443204a0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
                    ),
                    start: 571,
                    end: 580,
                    as_str(): "return 42",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb1443204a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
        ),
        start: 128,
        end: 583,
        as_str(): "fn main() -> u64 {\n    // all of these should be okay, since\n    // the branches that would have type errors abort control flow.\n    let x = if true { 42u64 } else { revert(0) };\n    let x: u64 = local_panic::<u64>();\n    let x = if let Result::Ok(ok) = Result::Ok::<u64, u64>(5) {\n        ok\n    } else {\n        local_panic::<u64>()\n    };\n    let x = if true {\n        Result::Err::<u64, u32>(12)\n    } else {\n        return 10;\n    };\n    return 42;\n}",
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
        src (ptr): 0x00007fb1443204a0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRtxQZet/abort_control_flow_good/src/main.sw",
        ),
        start: 141,
        end: 144,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

