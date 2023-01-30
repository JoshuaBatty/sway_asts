TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe066b453c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
            ),
            start: 16,
            end: 17,
            as_str(): "A",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe066b453c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                    ),
                    start: 24,
                    end: 26,
                    as_str(): "f1",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                9112,
            ),
            initial_type_id: TypeId(
                31630,
            ),
            span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 24,
                end: 35,
                as_str(): "f1: Address",
            },
            type_span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 28,
                end: 35,
                as_str(): "Address",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe066b453c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                    ),
                    start: 41,
                    end: 43,
                    as_str(): "f2",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7861,
            ),
            initial_type_id: TypeId(
                31631,
            ),
            span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 41,
                end: 55,
                as_str(): "f2: ContractId",
            },
            type_span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 45,
                end: 55,
                as_str(): "ContractId",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe066b453c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                    ),
                    start: 61,
                    end: 63,
                    as_str(): "f3",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                9181,
            ),
            initial_type_id: TypeId(
                31632,
            ),
            span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 61,
                end: 73,
                as_str(): "f3: Identity",
            },
            type_span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 65,
                end: 73,
                as_str(): "Identity",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe066b453c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                    ),
                    start: 79,
                    end: 81,
                    as_str(): "f4",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                31635,
            ),
            initial_type_id: TypeId(
                31633,
            ),
            span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 79,
                end: 90,
                as_str(): "f4: Vec<u8>",
            },
            type_span: Span {
                src (ptr): 0x00007fe066b453c0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                ),
                start: 83,
                end: 90,
                as_str(): "Vec<u8>",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe066b453c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
        ),
        start: 9,
        end: 93,
        as_str(): "struct A {\n    f1: Address,\n    f2: ContractId,\n    f3: Identity,\n    f4: Vec<u8>,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe066b453c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
            ),
            start: 98,
            end: 101,
            as_str(): "foo",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
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
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 110,
                                        end: 116,
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
                                            src (ptr): 0x00007fe061ee1820,
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
                                        expression: Literal(
                                            Boolean(
                                                true,
                                            ),
                                        ),
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 117,
                                            end: 121,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13565,
                                Span {
                                    src (ptr): 0x00007fe061ee1820,
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
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 110,
                                        end: 116,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31963,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe066b453c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                            ),
                            start: 110,
                            end: 122,
                            as_str(): "assert(true)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe066b453c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                    ),
                    start: 110,
                    end: 122,
                    as_str(): "assert(true)",
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
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 128,
                                        end: 135,
                                        as_str(): "require",
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
                                            src (ptr): 0x00007fe062733fc0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                            ),
                                            start: 1213,
                                            end: 1222,
                                            as_str(): "condition",
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
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 136,
                                            end: 140,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe062733fc0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                            ),
                                            start: 1230,
                                            end: 1235,
                                            as_str(): "value",
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
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 142,
                                            end: 143,
                                            as_str(): "0",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13570,
                                Span {
                                    src (ptr): 0x00007fe062733fc0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/revert.sw",
                                    ),
                                    start: 1195,
                                    end: 1327,
                                    as_str(): "pub fn require<T>(condition: bool, value: T) {\n    if !condition {\n        log(value);\n        revert(FAILED_REQUIRE_SIGNAL)\n    }\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 128,
                                        end: 135,
                                        as_str(): "require",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31969,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe066b453c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                            ),
                            start: 128,
                            end: 144,
                            as_str(): "require(true, 0)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe066b453c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                    ),
                    start: 128,
                    end: 144,
                    as_str(): "require(true, 0)",
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
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 156,
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
                                            src (ptr): 0x00007fe062733fc0,
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
                                            src (ptr): 0x00007fe066b453c0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                            ),
                                            start: 157,
                                            end: 158,
                                            as_str(): "0",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13572,
                                Span {
                                    src (ptr): 0x00007fe062733fc0,
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
                                        src (ptr): 0x00007fe066b453c0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                                        ),
                                        start: 150,
                                        end: 156,
                                        as_str(): "revert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31973,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe066b453c0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                            ),
                            start: 150,
                            end: 159,
                            as_str(): "revert(0)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe066b453c0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
                    ),
                    start: 150,
                    end: 159,
                    as_str(): "revert(0)",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe066b453c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
        ),
        start: 95,
        end: 162,
        as_str(): "fn foo() {\n    assert(true);\n    require(true, 0);\n    revert(0);\n}",
    },
    attributes: {},
    return_type: TypeId(
        31960,
    ),
    initial_return_type: TypeId(
        31959,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe066b453c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
        ),
        start: 95,
        end: 103,
        as_str(): "fn foo()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe066b453c0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
            ),
            start: 167,
            end: 171,
            as_str(): "main",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe066b453c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
        ),
        start: 164,
        end: 177,
        as_str(): "fn main() {\n}",
    },
    attributes: {},
    return_type: TypeId(
        31976,
    ),
    initial_return_type: TypeId(
        31975,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe066b453c0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRE1IFdk/prelude_access2/src/main.sw",
        ),
        start: 164,
        end: 173,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

