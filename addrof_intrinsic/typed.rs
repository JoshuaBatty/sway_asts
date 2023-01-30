



TyConstantDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14ccd5c90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
            ),
            start: 97,
            end: 99,
            as_str(): "B1",
        },
        is_raw_ident: false,
    },
    value: TyExpression {
        expression: StructExpression {
            struct_name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb14cfa9940,
                    path: Some(
                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/address.sw",
                    ),
                    start: 188,
                    end: 195,
                    as_str(): "Address",
                },
                is_raw_ident: false,
            },
            fields: [
                TyStructExpressionField {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 116,
                            end: 121,
                            as_str(): "value",
                        },
                        is_raw_ident: false,
                    },
                    value: TyExpression {
                        expression: Literal(
                            B256(
                                [
                                    1,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    0,
                                    16,
                                ],
                            ),
                        ),
                        return_type: TypeId(
                            59,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 123,
                            end: 189,
                            as_str(): "0x0100000000000000000000000000000000000000000000000000000000000010",
                        },
                    },
                },
            ],
            span: Span {
                src (ptr): 0x00007fb14ccd5c90,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                ),
                start: 102,
                end: 109,
                as_str(): "Address",
            },
        },
        return_type: TypeId(
            9112,
        ),
        span: Span {
            src (ptr): 0x00007fb14ccd5c90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
            ),
            start: 102,
            end: 191,
            as_str(): "Address {\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n}",
        },
    },
    visibility: Private,
    return_type: TypeId(
        9112,
    ),
    is_configurable: false,
    attributes: {},
    type_ascription_span: None,
    span: Span {
        src (ptr): 0x00007fb14ccd5c90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
        ),
        start: 91,
        end: 192,
        as_str(): "const B1 = Address {\n    value: 0x0100000000000000000000000000000000000000000000000000000000000010\n};",
    },
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14ccd5c90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
            ),
            start: 201,
            end: 208,
            as_str(): "addr_of",
        },
        is_raw_ident: false,
    },
    body: TyCodeBlock {
        contents: [
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
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 240,
                                                    end: 241,
                                                    as_str(): "!",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 240,
                                                    end: 241,
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 240,
                                                end: 241,
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
                                                    src (ptr): 0x00007fb14cea6e50,
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
                                                expression: IntrinsicFunction(
                                                    TyIntrinsicFunctionKind {
                                                        kind: IsReferenceType,
                                                        arguments: [],
                                                        type_arguments: [
                                                            TypeArgument {
                                                                type_id: TypeId(
                                                                    31636,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    31640,
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 263,
                                                                    end: 264,
                                                                    as_str(): "T",
                                                                },
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 241,
                                                            end: 267,
                                                            as_str(): "__is_reference_type::<T>()",
                                                        },
                                                    },
                                                ),
                                                return_type: TypeId(
                                                    71,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 241,
                                                    end: 267,
                                                    as_str(): "__is_reference_type::<T>()",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        13315,
                                        Span {
                                            src (ptr): 0x00007fb14cea6e50,
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 240,
                                                end: 241,
                                                as_str(): "!",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 240,
                                    end: 267,
                                    as_str(): "!__is_reference_type::<T>()",
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 278,
                                                                        end: 284,
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
                                                                            src (ptr): 0x00007fb1450c5bf0,
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 285,
                                                                            end: 286,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13317,
                                                                Span {
                                                                    src (ptr): 0x00007fb1450c5bf0,
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 278,
                                                                        end: 284,
                                                                        as_str(): "revert",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            31645,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 278,
                                                            end: 287,
                                                            as_str(): "revert(0)",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 278,
                                                    end: 287,
                                                    as_str(): "revert(0)",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7215,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 268,
                                    end: 294,
                                    as_str(): "{\n        revert(0);\n    }",
                                },
                            },
                            else: None,
                        },
                        return_type: TypeId(
                            31647,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 237,
                            end: 294,
                            as_str(): "if !__is_reference_type::<T>() {\n        revert(0);\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 237,
                    end: 294,
                    as_str(): "if !__is_reference_type::<T>() {\n        revert(0);\n    }",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: AsmExpression {
                            registers: [
                                TyAsmRegisterDeclaration {
                                    initializer: Some(
                                        TyExpression {
                                            expression: VariableExpression {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 212,
                                                        end: 215,
                                                        as_str(): "val",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 308,
                                                    end: 311,
                                                    as_str(): "val",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                31636,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 308,
                                                end: 311,
                                                as_str(): "val",
                                            },
                                        },
                                    ),
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 303,
                                            end: 306,
                                            as_str(): "ptr",
                                        },
                                        is_raw_ident: false,
                                    },
                                },
                            ],
                            body: [],
                            returns: Some(
                                (
                                    AsmRegister {
                                        name: "ptr",
                                    },
                                    Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 323,
                                        end: 326,
                                        as_str(): "ptr",
                                    },
                                ),
                            ),
                            whole_block_span: Span {
                                src (ptr): 0x00007fb14ccd5c90,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                ),
                                start: 299,
                                end: 341,
                                as_str(): "asm(ptr: val) {\n        ptr: raw_ptr\n    }",
                            },
                        },
                        return_type: TypeId(
                            20,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 299,
                            end: 341,
                            as_str(): "asm(ptr: val) {\n        ptr: raw_ptr\n    }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 299,
                    end: 341,
                    as_str(): "asm(ptr: val) {\n        ptr: raw_ptr\n    }",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 212,
                    end: 215,
                    as_str(): "val",
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
                31636,
            ),
            initial_type_id: TypeId(
                31637,
            ),
            type_span: Span {
                src (ptr): 0x00007fb14ccd5c90,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                ),
                start: 217,
                end: 218,
                as_str(): "T",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14ccd5c90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
        ),
        start: 194,
        end: 343,
        as_str(): "pub fn addr_of<T>(val: T) -> raw_ptr {\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
    },
    attributes: {},
    return_type: TypeId(
        20,
    ),
    initial_return_type: TypeId(
        20,
    ),
    type_parameters: [
        T: TypeId(31636),
    ],
    return_type_span: Span {
        src (ptr): 0x00007fb14ccd5c90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
        ),
        start: 223,
        end: 230,
        as_str(): "raw_ptr",
    },
    visibility: Public,
    is_contract_call: false,
    purity: Pure,
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14ccd5c90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
            ),
            start: 350,
            end: 351,
            as_str(): "X",
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
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 359,
                    end: 360,
                    as_str(): "A",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                33,
            ),
            initial_type_id: TypeId(
                33,
            ),
            type_span: Span {
                src (ptr): 0x00007fb14ccd5c90,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                ),
                start: 362,
                end: 365,
                as_str(): "u32",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fb14ccd5c90,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                ),
                start: 359,
                end: 365,
                as_str(): "A: u32",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 372,
                    end: 373,
                    as_str(): "B",
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
                src (ptr): 0x00007fb14ccd5c90,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                ),
                start: 375,
                end: 378,
                as_str(): "u64",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fb14ccd5c90,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                ),
                start: 372,
                end: 378,
                as_str(): "B: u64",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fb14ccd5c90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
        ),
        start: 345,
        end: 381,
        as_str(): "enum X {\n     A: u32,\n     B: u64,\n}",
    },
    visibility: Private,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fb14ccd5c90,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
            ),
            start: 386,
            end: 390,
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
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 403,
                                    end: 409,
                                    as_str(): "sender",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb14e643470,
                                                path: Some(
                                                    "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                ),
                                                start: 277,
                                                end: 285,
                                                as_str(): "Identity",
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
                                                        src (ptr): 0x00007fb14e643470,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                        ),
                                                        start: 292,
                                                        end: 299,
                                                        as_str(): "Address",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    9112,
                                                ),
                                                initial_type_id: TypeId(
                                                    9178,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb14e643470,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 301,
                                                    end: 308,
                                                    as_str(): "Address",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb14e643470,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 292,
                                                    end: 308,
                                                    as_str(): "Address: Address",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14e643470,
                                                        path: Some(
                                                            "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                        ),
                                                        start: 314,
                                                        end: 324,
                                                        as_str(): "ContractId",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7861,
                                                ),
                                                initial_type_id: TypeId(
                                                    9179,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb14e643470,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 326,
                                                    end: 336,
                                                    as_str(): "ContractId",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb14e643470,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                                    ),
                                                    start: 314,
                                                    end: 336,
                                                    as_str(): "ContractId: ContractId",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb14e643470,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                            ),
                                            start: 268,
                                            end: 339,
                                            as_str(): "pub enum Identity {\n    Address: Address,\n    ContractId: ContractId,\n}",
                                        },
                                        visibility: Public,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14e643470,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/identity.sw",
                                            ),
                                            start: 292,
                                            end: 299,
                                            as_str(): "Address",
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 97,
                                                        end: 99,
                                                        as_str(): "B1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 430,
                                                    end: 432,
                                                    as_str(): "B1",
                                                },
                                                mutability: Immutable,
                                            },
                                            return_type: TypeId(
                                                9112,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 430,
                                                end: 432,
                                                as_str(): "B1",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 412,
                                        end: 420,
                                        as_str(): "Identity",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 422,
                                        end: 429,
                                        as_str(): "Address",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 412,
                                            end: 429,
                                            as_str(): "Identity::Address",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    9181,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 422,
                                    end: 429,
                                    as_str(): "Address",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                9181,
                            ),
                            type_ascription: TypeId(
                                31652,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 399,
                    end: 434,
                    as_str(): "let sender = Identity::Address(B1);",
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 439,
                                        end: 445,
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
                                            src (ptr): 0x00007fb14d20f720,
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 465,
                                                            end: 467,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 465,
                                                            end: 467,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 465,
                                                        end: 467,
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
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3672,
                                                            end: 3676,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: AddrOf,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 403,
                                                                                    end: 409,
                                                                                    as_str(): "sender",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 457,
                                                                                end: 463,
                                                                                as_str(): "sender",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            9181,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 457,
                                                                            end: 463,
                                                                            as_str(): "sender",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 447,
                                                                    end: 464,
                                                                    as_str(): "__addr_of(sender)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 447,
                                                            end: 464,
                                                            as_str(): "__addr_of(sender)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3678,
                                                            end: 3683,
                                                            as_str(): "other",
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 468,
                                                                        end: 475,
                                                                        as_str(): "addr_of",
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 212,
                                                                            end: 215,
                                                                            as_str(): "val",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 403,
                                                                                    end: 409,
                                                                                    as_str(): "sender",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 476,
                                                                                end: 482,
                                                                                as_str(): "sender",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            9181,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 476,
                                                                            end: 482,
                                                                            as_str(): "sender",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13325,
                                                                Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 343,
                                                                    as_str(): "pub fn addr_of<T>(val: T) -> raw_ptr {\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 468,
                                                                        end: 475,
                                                                        as_str(): "addr_of",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 468,
                                                            end: 483,
                                                            as_str(): "addr_of(sender)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13326,
                                                Span {
                                                    src (ptr): 0x00007fb14cea6e50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3666,
                                                    end: 3732,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 465,
                                                        end: 467,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 447,
                                            end: 483,
                                            as_str(): "__addr_of(sender) == addr_of(sender)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13327,
                                Span {
                                    src (ptr): 0x00007fb14d20f720,
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 439,
                                        end: 445,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31661,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 439,
                            end: 484,
                            as_str(): "assert (__addr_of(sender) == addr_of(sender))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 439,
                    end: 484,
                    as_str(): "assert (__addr_of(sender) == addr_of(sender))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 495,
                                    end: 496,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 350,
                                                end: 351,
                                                as_str(): "X",
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 359,
                                                        end: 360,
                                                        as_str(): "A",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    33,
                                                ),
                                                initial_type_id: TypeId(
                                                    33,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 362,
                                                    end: 365,
                                                    as_str(): "u32",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 359,
                                                    end: 365,
                                                    as_str(): "A: u32",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 372,
                                                        end: 373,
                                                        as_str(): "B",
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
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 375,
                                                    end: 378,
                                                    as_str(): "u64",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 372,
                                                    end: 378,
                                                    as_str(): "B: u64",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 345,
                                            end: 381,
                                            as_str(): "enum X {\n     A: u32,\n     B: u64,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 359,
                                            end: 360,
                                            as_str(): "A",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 0,
                                    contents: Some(
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 504,
                                                end: 505,
                                                as_str(): "2",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 499,
                                        end: 500,
                                        as_str(): "X",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 502,
                                        end: 503,
                                        as_str(): "A",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 499,
                                            end: 503,
                                            as_str(): "X::A",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31663,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 502,
                                    end: 503,
                                    as_str(): "A",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31663,
                            ),
                            type_ascription: TypeId(
                                31662,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 491,
                    end: 507,
                    as_str(): "let x = X::A(2);",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 516,
                                    end: 517,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 350,
                                                end: 351,
                                                as_str(): "X",
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 359,
                                                        end: 360,
                                                        as_str(): "A",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    33,
                                                ),
                                                initial_type_id: TypeId(
                                                    33,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 362,
                                                    end: 365,
                                                    as_str(): "u32",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 359,
                                                    end: 365,
                                                    as_str(): "A: u32",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 372,
                                                        end: 373,
                                                        as_str(): "B",
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
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 375,
                                                    end: 378,
                                                    as_str(): "u64",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fb14ccd5c90,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                    ),
                                                    start: 372,
                                                    end: 378,
                                                    as_str(): "B: u64",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 345,
                                            end: 381,
                                            as_str(): "enum X {\n     A: u32,\n     B: u64,\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 372,
                                            end: 373,
                                            as_str(): "B",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 1,
                                    contents: Some(
                                        TyExpression {
                                            expression: Literal(
                                                U64(
                                                    22,
                                                ),
                                            ),
                                            return_type: TypeId(
                                                21,
                                            ),
                                            span: Span {
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 525,
                                                end: 527,
                                                as_str(): "22",
                                            },
                                        },
                                    ),
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 520,
                                        end: 521,
                                        as_str(): "X",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 523,
                                        end: 524,
                                        as_str(): "B",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 520,
                                            end: 524,
                                            as_str(): "X::B",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    31663,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 523,
                                    end: 524,
                                    as_str(): "B",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31663,
                            ),
                            type_ascription: TypeId(
                                31666,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 512,
                    end: 529,
                    as_str(): "let y = X::B(22);",
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 534,
                                        end: 540,
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
                                            src (ptr): 0x00007fb14d20f720,
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 554,
                                                            end: 556,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 554,
                                                            end: 556,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 554,
                                                        end: 556,
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
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3672,
                                                            end: 3676,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: AddrOf,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 495,
                                                                                    end: 496,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 551,
                                                                                end: 552,
                                                                                as_str(): "x",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31663,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 551,
                                                                            end: 552,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 541,
                                                                    end: 553,
                                                                    as_str(): "__addr_of(x)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 541,
                                                            end: 553,
                                                            as_str(): "__addr_of(x)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3678,
                                                            end: 3683,
                                                            as_str(): "other",
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 557,
                                                                        end: 564,
                                                                        as_str(): "addr_of",
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 212,
                                                                            end: 215,
                                                                            as_str(): "val",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 495,
                                                                                    end: 496,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 565,
                                                                                end: 566,
                                                                                as_str(): "x",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31663,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 565,
                                                                            end: 566,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13334,
                                                                Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 343,
                                                                    as_str(): "pub fn addr_of<T>(val: T) -> raw_ptr {\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 557,
                                                                        end: 564,
                                                                        as_str(): "addr_of",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 557,
                                                            end: 567,
                                                            as_str(): "addr_of(x)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13335,
                                                Span {
                                                    src (ptr): 0x00007fb14cea6e50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3666,
                                                    end: 3732,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 554,
                                                        end: 556,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 541,
                                            end: 567,
                                            as_str(): "__addr_of(x) == addr_of(x)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13336,
                                Span {
                                    src (ptr): 0x00007fb14d20f720,
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 534,
                                        end: 540,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31676,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 534,
                            end: 568,
                            as_str(): "assert(__addr_of(x) == addr_of(x))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 534,
                    end: 568,
                    as_str(): "assert(__addr_of(x) == addr_of(x))",
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 574,
                                        end: 580,
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
                                            src (ptr): 0x00007fb14d20f720,
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 594,
                                                            end: 596,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 594,
                                                            end: 596,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ],
                                                suffix: BaseIdent {
                                                    name_override_opt: Some(
                                                        "neq",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 594,
                                                        end: 596,
                                                        as_str(): "!=",
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
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2832,
                                                            end: 2836,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: AddrOf,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 495,
                                                                                    end: 496,
                                                                                    as_str(): "x",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 591,
                                                                                end: 592,
                                                                                as_str(): "x",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31663,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 591,
                                                                            end: 592,
                                                                            as_str(): "x",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 581,
                                                                    end: 593,
                                                                    as_str(): "__addr_of(x)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 581,
                                                            end: 593,
                                                            as_str(): "__addr_of(x)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 2838,
                                                            end: 2843,
                                                            as_str(): "other",
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 597,
                                                                        end: 604,
                                                                        as_str(): "addr_of",
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 212,
                                                                            end: 215,
                                                                            as_str(): "val",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 516,
                                                                                    end: 517,
                                                                                    as_str(): "y",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 605,
                                                                                end: 606,
                                                                                as_str(): "y",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31663,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 605,
                                                                            end: 606,
                                                                            as_str(): "y",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13341,
                                                                Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 343,
                                                                    as_str(): "pub fn addr_of<T>(val: T) -> raw_ptr {\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 597,
                                                                        end: 604,
                                                                        as_str(): "addr_of",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 597,
                                                            end: 607,
                                                            as_str(): "addr_of(y)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13342,
                                                Span {
                                                    src (ptr): 0x00007fb14cea6e50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 2825,
                                                    end: 2897,
                                                    as_str(): "fn neq(self, other: Self) -> bool {\n        (self.eq(other)).not()\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 594,
                                                        end: 596,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 581,
                                            end: 607,
                                            as_str(): "__addr_of(x) != addr_of(y)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13343,
                                Span {
                                    src (ptr): 0x00007fb14d20f720,
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 574,
                                        end: 580,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31684,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 574,
                            end: 608,
                            as_str(): "assert(__addr_of(x) != addr_of(y))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 574,
                    end: 608,
                    as_str(): "assert(__addr_of(x) != addr_of(y))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 619,
                                    end: 620,
                                    as_str(): "a",
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 624,
                                                end: 625,
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 626,
                                                end: 627,
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 628,
                                                end: 629,
                                                as_str(): "3",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    31693,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 623,
                                    end: 630,
                                    as_str(): "[1,2,3]",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31693,
                            ),
                            type_ascription: TypeId(
                                31685,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 615,
                    end: 631,
                    as_str(): "let a = [1,2,3];",
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 636,
                                        end: 642,
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
                                            src (ptr): 0x00007fb14d20f720,
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 656,
                                                            end: 658,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 656,
                                                            end: 658,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 656,
                                                        end: 658,
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
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3672,
                                                            end: 3676,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: AddrOf,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 619,
                                                                                    end: 620,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 653,
                                                                                end: 654,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31698,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 653,
                                                                            end: 654,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 643,
                                                                    end: 655,
                                                                    as_str(): "__addr_of(a)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 643,
                                                            end: 655,
                                                            as_str(): "__addr_of(a)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3678,
                                                            end: 3683,
                                                            as_str(): "other",
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 659,
                                                                        end: 666,
                                                                        as_str(): "addr_of",
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 212,
                                                                            end: 215,
                                                                            as_str(): "val",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 619,
                                                                                    end: 620,
                                                                                    as_str(): "a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 667,
                                                                                end: 668,
                                                                                as_str(): "a",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31702,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 667,
                                                                            end: 668,
                                                                            as_str(): "a",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13348,
                                                                Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 343,
                                                                    as_str(): "pub fn addr_of<T>(val: T) -> raw_ptr {\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 659,
                                                                        end: 666,
                                                                        as_str(): "addr_of",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 659,
                                                            end: 669,
                                                            as_str(): "addr_of(a)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13349,
                                                Span {
                                                    src (ptr): 0x00007fb14cea6e50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3666,
                                                    end: 3732,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 656,
                                                        end: 658,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 643,
                                            end: 669,
                                            as_str(): "__addr_of(a) == addr_of(a)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13350,
                                Span {
                                    src (ptr): 0x00007fb14d20f720,
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 636,
                                        end: 642,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31703,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 636,
                            end: 670,
                            as_str(): "assert(__addr_of(a) == addr_of(a))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 636,
                    end: 670,
                    as_str(): "assert(__addr_of(a) == addr_of(a))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 681,
                                    end: 682,
                                    as_str(): "b",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    String(
                                        Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 686,
                                            end: 691,
                                            as_str(): "hello",
                                        },
                                    ),
                                ),
                                return_type: TypeId(
                                    31705,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 685,
                                    end: 692,
                                    as_str(): "\"hello\"",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31705,
                            ),
                            type_ascription: TypeId(
                                31704,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 677,
                    end: 693,
                    as_str(): "let b = \"hello\";",
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 698,
                                        end: 704,
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
                                            src (ptr): 0x00007fb14d20f720,
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 718,
                                                            end: 720,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 718,
                                                            end: 720,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 718,
                                                        end: 720,
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
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3672,
                                                            end: 3676,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: AddrOf,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 681,
                                                                                    end: 682,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 715,
                                                                                end: 716,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31705,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 715,
                                                                            end: 716,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 705,
                                                                    end: 717,
                                                                    as_str(): "__addr_of(b)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 705,
                                                            end: 717,
                                                            as_str(): "__addr_of(b)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3678,
                                                            end: 3683,
                                                            as_str(): "other",
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 721,
                                                                        end: 728,
                                                                        as_str(): "addr_of",
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 212,
                                                                            end: 215,
                                                                            as_str(): "val",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 681,
                                                                                    end: 682,
                                                                                    as_str(): "b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 729,
                                                                                end: 730,
                                                                                as_str(): "b",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31705,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 729,
                                                                            end: 730,
                                                                            as_str(): "b",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13355,
                                                                Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 343,
                                                                    as_str(): "pub fn addr_of<T>(val: T) -> raw_ptr {\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 721,
                                                                        end: 728,
                                                                        as_str(): "addr_of",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 721,
                                                            end: 731,
                                                            as_str(): "addr_of(b)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13356,
                                                Span {
                                                    src (ptr): 0x00007fb14cea6e50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3666,
                                                    end: 3732,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 718,
                                                        end: 720,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 705,
                                            end: 731,
                                            as_str(): "__addr_of(b) == addr_of(b)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13357,
                                Span {
                                    src (ptr): 0x00007fb14d20f720,
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 698,
                                        end: 704,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31713,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 698,
                            end: 732,
                            as_str(): "assert(__addr_of(b) == addr_of(b))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 698,
                    end: 732,
                    as_str(): "assert(__addr_of(b) == addr_of(b))",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 743,
                                    end: 744,
                                    as_str(): "c",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Tuple {
                                    fields: [
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 748,
                                                end: 749,
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
                                                src (ptr): 0x00007fb14ccd5c90,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                ),
                                                start: 751,
                                                end: 752,
                                                as_str(): "2",
                                            },
                                        },
                                    ],
                                },
                                return_type: TypeId(
                                    31720,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb14ccd5c90,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                    ),
                                    start: 747,
                                    end: 753,
                                    as_str(): "(1, 2)",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                31720,
                            ),
                            type_ascription: TypeId(
                                31714,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 739,
                    end: 754,
                    as_str(): "let c = (1, 2);",
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 759,
                                        end: 765,
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
                                            src (ptr): 0x00007fb14d20f720,
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
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 779,
                                                            end: 781,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 779,
                                                            end: 781,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 779,
                                                        end: 781,
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
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3672,
                                                            end: 3676,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: IntrinsicFunction(
                                                            TyIntrinsicFunctionKind {
                                                                kind: AddrOf,
                                                                arguments: [
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 743,
                                                                                    end: 744,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 776,
                                                                                end: 777,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31725,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 776,
                                                                            end: 777,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ],
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 766,
                                                                    end: 778,
                                                                    as_str(): "__addr_of(c)",
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 766,
                                                            end: 778,
                                                            as_str(): "__addr_of(c)",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fb14cea6e50,
                                                            path: Some(
                                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                            ),
                                                            start: 3678,
                                                            end: 3683,
                                                            as_str(): "other",
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
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 782,
                                                                        end: 789,
                                                                        as_str(): "addr_of",
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
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 212,
                                                                            end: 215,
                                                                            as_str(): "val",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    TyExpression {
                                                                        expression: VariableExpression {
                                                                            name: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                    ),
                                                                                    start: 743,
                                                                                    end: 744,
                                                                                    as_str(): "c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb14ccd5c90,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                                ),
                                                                                start: 790,
                                                                                end: 791,
                                                                                as_str(): "c",
                                                                            },
                                                                            mutability: Immutable,
                                                                        },
                                                                        return_type: TypeId(
                                                                            31729,
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb14ccd5c90,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                            ),
                                                                            start: 790,
                                                                            end: 791,
                                                                            as_str(): "c",
                                                                        },
                                                                    },
                                                                ),
                                                            ],
                                                            function_decl_id: DeclId(
                                                                13362,
                                                                Span {
                                                                    src (ptr): 0x00007fb14ccd5c90,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                    ),
                                                                    start: 194,
                                                                    end: 343,
                                                                    as_str(): "pub fn addr_of<T>(val: T) -> raw_ptr {\n    if !__is_reference_type::<T>() {\n        revert(0);\n    }\n    asm(ptr: val) {\n        ptr: raw_ptr\n    }\n}",
                                                                },
                                                            ),
                                                            self_state_idx: None,
                                                            selector: None,
                                                            type_binding: Some(
                                                                TypeBinding {
                                                                    inner: (),
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb14ccd5c90,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                                        ),
                                                                        start: 782,
                                                                        end: 789,
                                                                        as_str(): "addr_of",
                                                                    },
                                                                },
                                                            ),
                                                        },
                                                        return_type: TypeId(
                                                            20,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb14ccd5c90,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                            ),
                                                            start: 782,
                                                            end: 792,
                                                            as_str(): "addr_of(c)",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                13363,
                                                Span {
                                                    src (ptr): 0x00007fb14cea6e50,
                                                    path: Some(
                                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-core/src/ops.sw",
                                                    ),
                                                    start: 3666,
                                                    end: 3732,
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
                                                        src (ptr): 0x00007fb14ccd5c90,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                                        ),
                                                        start: 779,
                                                        end: 781,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb14ccd5c90,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                            ),
                                            start: 766,
                                            end: 792,
                                            as_str(): "__addr_of(c) == addr_of(c)",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                13364,
                                Span {
                                    src (ptr): 0x00007fb14d20f720,
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
                                        src (ptr): 0x00007fb14ccd5c90,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                                        ),
                                        start: 759,
                                        end: 765,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            31730,
                        ),
                        span: Span {
                            src (ptr): 0x00007fb14ccd5c90,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                            ),
                            start: 759,
                            end: 793,
                            as_str(): "assert(__addr_of(c) == addr_of(c))",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fb14ccd5c90,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
                    ),
                    start: 759,
                    end: 793,
                    as_str(): "assert(__addr_of(c) == addr_of(c))",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb14ccd5c90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
        ),
        start: 383,
        end: 796,
        as_str(): "fn main() {\n    let sender = Identity::Address(B1);\n    assert (__addr_of(sender) == addr_of(sender));\n\n    let x = X::A(2);\n    let y = X::B(22);\n    assert(__addr_of(x) == addr_of(x));\n    assert(__addr_of(x) != addr_of(y));\n\n    let a = [1,2,3];\n    assert(__addr_of(a) == addr_of(a));\n\n    let b = \"hello\";\n    assert(__addr_of(b) == addr_of(b));\n\n    let c = (1, 2);\n    assert(__addr_of(c) == addr_of(c));\n}",
    },
    attributes: {},
    return_type: TypeId(
        31651,
    ),
    initial_return_type: TypeId(
        31650,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fb14ccd5c90,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRA8cr3X/addrof_intrinsic/src/main.sw",
        ),
        start: 383,
        end: 392,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

