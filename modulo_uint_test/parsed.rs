[
    AstNode {
        content: UseStatement(
            UseStatement {
                call_path: [
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 13,
                            end: 16,
                            as_str(): "std",
                        },
                        is_raw_ident: false,
                    },
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 18,
                            end: 24,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ],
                import_type: Item(
                    BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 26,
                            end: 32,
                            as_str(): "assert",
                        },
                        is_raw_ident: false,
                    },
                ),
                is_absolute: false,
                alias: None,
            },
        ),
        span: Span {
            src (ptr): 0x00007fe082cf6db0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
            ),
            start: 9,
            end: 33,
            as_str(): "use std::assert::assert;",
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
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 38,
                            end: 42,
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
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 63,
                                                    end: 75,
                                                    as_str(): "uint64_test1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 77,
                                                    end: 80,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        100000000000,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 83,
                                                    end: 95,
                                                    as_str(): "100000000000",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 59,
                                    end: 96,
                                    as_str(): "let uint64_test1: u64 = 100000000000;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 105,
                                                    end: 117,
                                                    as_str(): "uint32_test1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 119,
                                                    end: 122,
                                                    as_str(): "u32",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1000000000,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 125,
                                                    end: 135,
                                                    as_str(): "1000000000",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 101,
                                    end: 136,
                                    as_str(): "let uint32_test1: u32 = 1000000000;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 145,
                                                    end: 157,
                                                    as_str(): "uint16_test1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Sixteen,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 159,
                                                    end: 162,
                                                    as_str(): "u16",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        10000,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 165,
                                                    end: 170,
                                                    as_str(): "10000",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 141,
                                    end: 171,
                                    as_str(): "let uint16_test1: u16 = 10000;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 180,
                                                    end: 191,
                                                    as_str(): "uint8_test1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Eight,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 193,
                                                    end: 195,
                                                    as_str(): "u8",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        100,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 198,
                                                    end: 201,
                                                    as_str(): "100",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 176,
                                    end: 202,
                                    as_str(): "let uint8_test1: u8 = 100;",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 252,
                                                                end: 258,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 252,
                                                        end: 258,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 281,
                                                                                        end: 283,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 281,
                                                                                        end: 283,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 281,
                                                                                    end: 283,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 281,
                                                                        end: 283,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 272,
                                                                                                        end: 273,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 272,
                                                                                                        end: 273,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 272,
                                                                                                    end: 273,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 272,
                                                                                        end: 273,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 259,
                                                                                                    end: 271,
                                                                                                    as_str(): "uint64_test1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 259,
                                                                                            end: 271,
                                                                                            as_str(): "uint64_test1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U64(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 274,
                                                                                            end: 280,
                                                                                            as_str(): "100u64",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 259,
                                                                            end: 280,
                                                                            as_str(): "uint64_test1 % 100u64",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 284,
                                                                            end: 285,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 259,
                                                            end: 285,
                                                            as_str(): "uint64_test1 % 100u64 == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 252,
                                            end: 286,
                                            as_str(): "assert(uint64_test1 % 100u64 == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 252,
                                    end: 286,
                                    as_str(): "assert(uint64_test1 % 100u64 == 0)",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 292,
                                                                end: 298,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 292,
                                                        end: 298,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 321,
                                                                                        end: 323,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 321,
                                                                                        end: 323,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 321,
                                                                                    end: 323,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 321,
                                                                        end: 323,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 312,
                                                                                                        end: 313,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 312,
                                                                                                        end: 313,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 312,
                                                                                                    end: 313,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 312,
                                                                                        end: 313,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 299,
                                                                                                    end: 311,
                                                                                                    as_str(): "uint32_test1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 299,
                                                                                            end: 311,
                                                                                            as_str(): "uint32_test1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U32(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 314,
                                                                                            end: 320,
                                                                                            as_str(): "100u32",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 299,
                                                                            end: 320,
                                                                            as_str(): "uint32_test1 % 100u32",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 324,
                                                                            end: 325,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 299,
                                                            end: 325,
                                                            as_str(): "uint32_test1 % 100u32 == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 292,
                                            end: 326,
                                            as_str(): "assert(uint32_test1 % 100u32 == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 292,
                                    end: 326,
                                    as_str(): "assert(uint32_test1 % 100u32 == 0)",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 332,
                                                                end: 338,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 332,
                                                        end: 338,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 361,
                                                                                        end: 363,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 361,
                                                                                        end: 363,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 361,
                                                                                    end: 363,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 361,
                                                                        end: 363,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 352,
                                                                                                        end: 353,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 352,
                                                                                                        end: 353,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 352,
                                                                                                    end: 353,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 352,
                                                                                        end: 353,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 339,
                                                                                                    end: 351,
                                                                                                    as_str(): "uint16_test1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 339,
                                                                                            end: 351,
                                                                                            as_str(): "uint16_test1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U16(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 354,
                                                                                            end: 360,
                                                                                            as_str(): "100u16",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 339,
                                                                            end: 360,
                                                                            as_str(): "uint16_test1 % 100u16",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 364,
                                                                            end: 365,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 339,
                                                            end: 365,
                                                            as_str(): "uint16_test1 % 100u16 == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 332,
                                            end: 366,
                                            as_str(): "assert(uint16_test1 % 100u16 == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 332,
                                    end: 366,
                                    as_str(): "assert(uint16_test1 % 100u16 == 0)",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 372,
                                                                end: 378,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 372,
                                                        end: 378,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 399,
                                                                                        end: 401,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 399,
                                                                                        end: 401,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 399,
                                                                                    end: 401,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 399,
                                                                        end: 401,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 391,
                                                                                                        end: 392,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 391,
                                                                                                        end: 392,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 391,
                                                                                                    end: 392,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 391,
                                                                                        end: 392,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 379,
                                                                                                    end: 390,
                                                                                                    as_str(): "uint8_test1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 379,
                                                                                            end: 390,
                                                                                            as_str(): "uint8_test1",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U8(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 393,
                                                                                            end: 398,
                                                                                            as_str(): "100u8",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 379,
                                                                            end: 398,
                                                                            as_str(): "uint8_test1 % 100u8",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                0,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 402,
                                                                            end: 403,
                                                                            as_str(): "0",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 379,
                                                            end: 403,
                                                            as_str(): "uint8_test1 % 100u8 == 0",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 372,
                                            end: 404,
                                            as_str(): "assert(uint8_test1 % 100u8 == 0)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 372,
                                    end: 404,
                                    as_str(): "assert(uint8_test1 % 100u8 == 0)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 415,
                                                    end: 427,
                                                    as_str(): "uint64_test2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                SixtyFour,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 429,
                                                    end: 432,
                                                    as_str(): "u64",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        100000000005,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 435,
                                                    end: 447,
                                                    as_str(): "100000000005",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 411,
                                    end: 448,
                                    as_str(): "let uint64_test2: u64 = 100000000005;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 457,
                                                    end: 469,
                                                    as_str(): "uint32_test2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                ThirtyTwo,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 471,
                                                    end: 474,
                                                    as_str(): "u32",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1000000005,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 477,
                                                    end: 487,
                                                    as_str(): "1000000005",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 453,
                                    end: 488,
                                    as_str(): "let uint32_test2: u32 = 1000000005;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 497,
                                                    end: 509,
                                                    as_str(): "uint16_test2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Sixteen,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 511,
                                                    end: 514,
                                                    as_str(): "u16",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        10005,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 517,
                                                    end: 522,
                                                    as_str(): "10005",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 493,
                                    end: 523,
                                    as_str(): "let uint16_test2: u16 = 10005;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    VariableDeclaration(
                                        VariableDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 532,
                                                    end: 543,
                                                    as_str(): "uint8_test2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            type_ascription: UnsignedInteger(
                                                Eight,
                                            ),
                                            type_ascription_span: Some(
                                                Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 545,
                                                    end: 547,
                                                    as_str(): "u8",
                                                },
                                            ),
                                            body: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        105,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe082cf6db0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                    ),
                                                    start: 550,
                                                    end: 553,
                                                    as_str(): "105",
                                                },
                                            },
                                            is_mutable: false,
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 528,
                                    end: 554,
                                    as_str(): "let uint8_test2: u8 = 105;",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 611,
                                                                end: 617,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 611,
                                                        end: 617,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 640,
                                                                                        end: 642,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 640,
                                                                                        end: 642,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 640,
                                                                                    end: 642,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 640,
                                                                        end: 642,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 631,
                                                                                                        end: 632,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 631,
                                                                                                        end: 632,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 631,
                                                                                                    end: 632,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 631,
                                                                                        end: 632,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 618,
                                                                                                    end: 630,
                                                                                                    as_str(): "uint64_test2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 618,
                                                                                            end: 630,
                                                                                            as_str(): "uint64_test2",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U64(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 633,
                                                                                            end: 639,
                                                                                            as_str(): "100u64",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 618,
                                                                            end: 639,
                                                                            as_str(): "uint64_test2 % 100u64",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 643,
                                                                            end: 644,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 618,
                                                            end: 644,
                                                            as_str(): "uint64_test2 % 100u64 == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 611,
                                            end: 645,
                                            as_str(): "assert(uint64_test2 % 100u64 == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 611,
                                    end: 645,
                                    as_str(): "assert(uint64_test2 % 100u64 == 5)",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 651,
                                                                end: 657,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 651,
                                                        end: 657,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 680,
                                                                                        end: 682,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 680,
                                                                                        end: 682,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 680,
                                                                                    end: 682,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 680,
                                                                        end: 682,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 671,
                                                                                                        end: 672,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 671,
                                                                                                        end: 672,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 671,
                                                                                                    end: 672,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 671,
                                                                                        end: 672,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 658,
                                                                                                    end: 670,
                                                                                                    as_str(): "uint32_test2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 658,
                                                                                            end: 670,
                                                                                            as_str(): "uint32_test2",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U32(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 673,
                                                                                            end: 679,
                                                                                            as_str(): "100u32",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 658,
                                                                            end: 679,
                                                                            as_str(): "uint32_test2 % 100u32",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 683,
                                                                            end: 684,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 658,
                                                            end: 684,
                                                            as_str(): "uint32_test2 % 100u32 == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 651,
                                            end: 685,
                                            as_str(): "assert(uint32_test2 % 100u32 == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 651,
                                    end: 685,
                                    as_str(): "assert(uint32_test2 % 100u32 == 5)",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 691,
                                                                end: 697,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 691,
                                                        end: 697,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 720,
                                                                                        end: 722,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 720,
                                                                                        end: 722,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 720,
                                                                                    end: 722,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 720,
                                                                        end: 722,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 711,
                                                                                                        end: 712,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 711,
                                                                                                        end: 712,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 711,
                                                                                                    end: 712,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 711,
                                                                                        end: 712,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 698,
                                                                                                    end: 710,
                                                                                                    as_str(): "uint16_test2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 698,
                                                                                            end: 710,
                                                                                            as_str(): "uint16_test2",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U16(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 713,
                                                                                            end: 719,
                                                                                            as_str(): "100u16",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 698,
                                                                            end: 719,
                                                                            as_str(): "uint16_test2 % 100u16",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 723,
                                                                            end: 724,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 698,
                                                            end: 724,
                                                            as_str(): "uint16_test2 % 100u16 == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 691,
                                            end: 725,
                                            as_str(): "assert(uint16_test2 % 100u16 == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 691,
                                    end: 725,
                                    as_str(): "assert(uint16_test2 % 100u16 == 5)",
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
                                                                src (ptr): 0x00007fe082cf6db0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                ),
                                                                start: 731,
                                                                end: 737,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe082cf6db0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                        ),
                                                        start: 731,
                                                        end: 737,
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
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 758,
                                                                                        end: 760,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 758,
                                                                                        end: 760,
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
                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                    ),
                                                                                    start: 758,
                                                                                    end: 760,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                        ),
                                                                        start: 758,
                                                                        end: 760,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
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
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 750,
                                                                                                        end: 751,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                                BaseIdent {
                                                                                                    name_override_opt: Some(
                                                                                                        "ops",
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                        ),
                                                                                                        start: 750,
                                                                                                        end: 751,
                                                                                                        as_str(): "%",
                                                                                                    },
                                                                                                    is_raw_ident: false,
                                                                                                },
                                                                                            ],
                                                                                            suffix: BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "modulo",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 750,
                                                                                                    end: 751,
                                                                                                    as_str(): "%",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            is_absolute: true,
                                                                                        },
                                                                                    },
                                                                                    type_arguments: [],
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe082cf6db0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                        ),
                                                                                        start: 750,
                                                                                        end: 751,
                                                                                        as_str(): "%",
                                                                                    },
                                                                                },
                                                                                contract_call_params: [],
                                                                                arguments: [
                                                                                    Expression {
                                                                                        kind: Variable(
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fe082cf6db0,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                                    ),
                                                                                                    start: 738,
                                                                                                    end: 749,
                                                                                                    as_str(): "uint8_test2",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 738,
                                                                                            end: 749,
                                                                                            as_str(): "uint8_test2",
                                                                                        },
                                                                                    },
                                                                                    Expression {
                                                                                        kind: Literal(
                                                                                            U8(
                                                                                                100,
                                                                                            ),
                                                                                        ),
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                                            ),
                                                                                            start: 752,
                                                                                            end: 757,
                                                                                            as_str(): "100u8",
                                                                                        },
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 738,
                                                                            end: 757,
                                                                            as_str(): "uint8_test2 % 100u8",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Literal(
                                                                            Numeric(
                                                                                5,
                                                                            ),
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fe082cf6db0,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                                            ),
                                                                            start: 761,
                                                                            end: 762,
                                                                            as_str(): "5",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe082cf6db0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                                            ),
                                                            start: 738,
                                                            end: 762,
                                                            as_str(): "uint8_test2 % 100u8 == 5",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 731,
                                            end: 763,
                                            as_str(): "assert(uint8_test2 % 100u8 == 5)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 731,
                                    end: 763,
                                    as_str(): "assert(uint8_test2 % 100u8 == 5)",
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
                                            src (ptr): 0x00007fe082cf6db0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                            ),
                                            start: 770,
                                            end: 774,
                                            as_str(): "true",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe082cf6db0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                                    ),
                                    start: 770,
                                    end: 774,
                                    as_str(): "true",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fe082cf6db0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                            ),
                            start: 53,
                            end: 776,
                            as_str(): "{\n    let uint64_test1: u64 = 100000000000;\n    let uint32_test1: u32 = 1000000000;\n    let uint16_test1: u16 = 10000;\n    let uint8_test1: u8 = 100;\n\n    // Ensure 0 remainder returns correctly\n    assert(uint64_test1 % 100u64 == 0);\n    assert(uint32_test1 % 100u32 == 0);\n    assert(uint16_test1 % 100u16 == 0);\n    assert(uint8_test1 % 100u8 == 0);\n\n    let uint64_test2: u64 = 100000000005;\n    let uint32_test2: u32 = 1000000005;\n    let uint16_test2: u16 = 10005;\n    let uint8_test2: u8 = 105;\n\n    // Ensure non zero remainder returns correctly\n    assert(uint64_test2 % 100u64 == 5);\n    assert(uint32_test2 % 100u32 == 5);\n    assert(uint16_test2 % 100u16 == 5);\n    assert(uint8_test2 % 100u8 == 5);\n\n    true\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fe082cf6db0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                        ),
                        start: 35,
                        end: 776,
                        as_str(): "fn main() -> bool {\n    let uint64_test1: u64 = 100000000000;\n    let uint32_test1: u32 = 1000000000;\n    let uint16_test1: u16 = 10000;\n    let uint8_test1: u8 = 100;\n\n    // Ensure 0 remainder returns correctly\n    assert(uint64_test1 % 100u64 == 0);\n    assert(uint32_test1 % 100u32 == 0);\n    assert(uint16_test1 % 100u16 == 0);\n    assert(uint8_test1 % 100u8 == 0);\n\n    let uint64_test2: u64 = 100000000005;\n    let uint32_test2: u32 = 1000000005;\n    let uint16_test2: u16 = 10005;\n    let uint8_test2: u8 = 105;\n\n    // Ensure non zero remainder returns correctly\n    assert(uint64_test2 % 100u64 == 5);\n    assert(uint32_test2 % 100u32 == 5);\n    assert(uint16_test2 % 100u16 == 5);\n    assert(uint8_test2 % 100u8 == 5);\n\n    true\n}",
                    },
                    return_type: Boolean,
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fe082cf6db0,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
                        ),
                        start: 48,
                        end: 52,
                        as_str(): "bool",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fe082cf6db0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRA7G8Mx/modulo_uint_test/src/main.sw",
            ),
            start: 35,
            end: 776,
            as_str(): "fn main() -> bool {\n    let uint64_test1: u64 = 100000000000;\n    let uint32_test1: u32 = 1000000000;\n    let uint16_test1: u16 = 10000;\n    let uint8_test1: u8 = 100;\n\n    // Ensure 0 remainder returns correctly\n    assert(uint64_test1 % 100u64 == 0);\n    assert(uint32_test1 % 100u32 == 0);\n    assert(uint16_test1 % 100u16 == 0);\n    assert(uint8_test1 % 100u8 == 0);\n\n    let uint64_test2: u64 = 100000000005;\n    let uint32_test2: u32 = 1000000005;\n    let uint16_test2: u16 = 10005;\n    let uint8_test2: u8 = 105;\n\n    // Ensure non zero remainder returns correctly\n    assert(uint64_test2 % 100u64 == 5);\n    assert(uint32_test2 % 100u32 == 5);\n    assert(uint16_test2 % 100u16 == 5);\n    assert(uint8_test2 % 100u8 == 5);\n\n    true\n}",
        },
    },
]
