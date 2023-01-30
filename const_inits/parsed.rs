[
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 15,
                            end: 22,
                            as_str(): "ETH_ID0",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: AmbiguousPathExpression(
                            AmbiguousPathExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: AmbiguousSuffix {
                                            before: TypeBinding {
                                                inner: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 25,
                                                        end: 35,
                                                        as_str(): "ContractId",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 25,
                                                    end: 35,
                                                    as_str(): "ContractId",
                                                },
                                            },
                                            suffix: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 37,
                                                    end: 41,
                                                    as_str(): "from",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 25,
                                        end: 41,
                                        as_str(): "ContractId::from",
                                    },
                                },
                                args: [
                                    Expression {
                                        kind: Literal(
                                            B256(
                                                [
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
                                                    0,
                                                    0,
                                                ],
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 42,
                                            end: 108,
                                            as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 25,
                            end: 109,
                            as_str(): "ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 9,
                        end: 110,
                        as_str(): "const ETH_ID0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 9,
            end: 110,
            as_str(): "const ETH_ID0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);",
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
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 114,
                            end: 133,
                            as_str(): "contract_id_wrapper",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: AmbiguousPathExpression(
                                            AmbiguousPathExpression {
                                                call_path_binding: TypeBinding {
                                                    inner: CallPath {
                                                        prefixes: [],
                                                        suffix: AmbiguousSuffix {
                                                            before: TypeBinding {
                                                                inner: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 163,
                                                                        end: 173,
                                                                        as_str(): "ContractId",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_arguments: [],
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 163,
                                                                    end: 173,
                                                                    as_str(): "ContractId",
                                                                },
                                                            },
                                                            suffix: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 175,
                                                                    end: 179,
                                                                    as_str(): "from",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 163,
                                                        end: 179,
                                                        as_str(): "ContractId::from",
                                                    },
                                                },
                                                args: [
                                                    Expression {
                                                        kind: Variable(
                                                            BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 180,
                                                                    end: 181,
                                                                    as_str(): "b",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 180,
                                                            end: 181,
                                                            as_str(): "b",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 163,
                                            end: 182,
                                            as_str(): "ContractId::from(b)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 163,
                                    end: 182,
                                    as_str(): "ContractId::from(b)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 157,
                            end: 184,
                            as_str(): "{\n    ContractId::from(b)\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 134,
                                    end: 135,
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
                            type_info: B256,
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 137,
                                end: 141,
                                as_str(): "b256",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 111,
                        end: 184,
                        as_str(): "fn contract_id_wrapper(b: b256) -> ContractId {\n    ContractId::from(b)\n}",
                    },
                    return_type: Custom {
                        name: BaseIdent {
                            name_override_opt: None,
                            span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 146,
                                end: 156,
                                as_str(): "ContractId",
                            },
                            is_raw_ident: false,
                        },
                        type_arguments: Some(
                            [],
                        ),
                    },
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 146,
                        end: 156,
                        as_str(): "ContractId",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 111,
            end: 184,
            as_str(): "fn contract_id_wrapper(b: b256) -> ContractId {\n    ContractId::from(b)\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 191,
                            end: 198,
                            as_str(): "ETH_ID1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: FunctionApplication(
                            FunctionApplicationExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 201,
                                                end: 220,
                                                as_str(): "contract_id_wrapper",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 201,
                                        end: 220,
                                        as_str(): "contract_id_wrapper",
                                    },
                                },
                                arguments: [
                                    Expression {
                                        kind: Literal(
                                            B256(
                                                [
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
                                                    0,
                                                    1,
                                                ],
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 221,
                                            end: 287,
                                            as_str(): "0x0000000000000000000000000000000000000000000000000000000000000001",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 201,
                            end: 288,
                            as_str(): "contract_id_wrapper(0x0000000000000000000000000000000000000000000000000000000000000001)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 185,
                        end: 289,
                        as_str(): "const ETH_ID1 = contract_id_wrapper(0x0000000000000000000000000000000000000000000000000000000000000001);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 185,
            end: 289,
            as_str(): "const ETH_ID1 = contract_id_wrapper(0x0000000000000000000000000000000000000000000000000000000000000001);",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 297,
                            end: 301,
                            as_str(): "TUP1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Tuple(
                            [
                                Expression {
                                    kind: Literal(
                                        Numeric(
                                            2,
                                        ),
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 305,
                                        end: 306,
                                        as_str(): "2",
                                    },
                                },
                                Expression {
                                    kind: Literal(
                                        Numeric(
                                            1,
                                        ),
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 308,
                                        end: 309,
                                        as_str(): "1",
                                    },
                                },
                                Expression {
                                    kind: Literal(
                                        Numeric(
                                            21,
                                        ),
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 311,
                                        end: 313,
                                        as_str(): "21",
                                    },
                                },
                            ],
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 304,
                            end: 314,
                            as_str(): "(2, 1, 21)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 291,
                        end: 315,
                        as_str(): "const TUP1 = (2, 1, 21);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 291,
            end: 315,
            as_str(): "const TUP1 = (2, 1, 21);",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 322,
                            end: 326,
                            as_str(): "ARR1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Array(
                            ArrayExpression {
                                contents: [
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 330,
                                            end: 331,
                                            as_str(): "1",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                2,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 333,
                                            end: 334,
                                            as_str(): "2",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                3,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 336,
                                            end: 337,
                                            as_str(): "3",
                                        },
                                    },
                                ],
                                length_span: None,
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 329,
                            end: 338,
                            as_str(): "[1, 2, 3]",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 316,
                        end: 339,
                        as_str(): "const ARR1 = [1, 2, 3];",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 316,
            end: 339,
            as_str(): "const ARR1 = [1, 2, 3];",
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
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 344,
                            end: 355,
                            as_str(): "tup_wrapper",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Tuple(
                                            [
                                                Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 406,
                                                                end: 407,
                                                                as_str(): "a",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 406,
                                                        end: 407,
                                                        as_str(): "a",
                                                    },
                                                },
                                                Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 409,
                                                                end: 410,
                                                                as_str(): "b",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 409,
                                                        end: 410,
                                                        as_str(): "b",
                                                    },
                                                },
                                                Expression {
                                                    kind: Variable(
                                                        BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 412,
                                                                end: 413,
                                                                as_str(): "c",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 412,
                                                        end: 413,
                                                        as_str(): "c",
                                                    },
                                                },
                                            ],
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 405,
                                            end: 414,
                                            as_str(): "(a, b, c)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 405,
                                    end: 414,
                                    as_str(): "(a, b, c)",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 399,
                            end: 416,
                            as_str(): "{\n    (a, b, c)\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 356,
                                    end: 357,
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 359,
                                end: 362,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 364,
                                    end: 365,
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 367,
                                end: 370,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 372,
                                    end: 373,
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 375,
                                end: 378,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 341,
                        end: 416,
                        as_str(): "fn tup_wrapper(a: u64, b: u64, c: u64) -> (u64, u64, u64) {\n    (a, b, c)\n}",
                    },
                    return_type: Tuple(
                        [
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 384,
                                    end: 387,
                                    as_str(): "u64",
                                },
                            },
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 389,
                                    end: 392,
                                    as_str(): "u64",
                                },
                            },
                            TypeArgument {
                                type_id: TypeId(
                                    21,
                                ),
                                initial_type_id: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 394,
                                    end: 397,
                                    as_str(): "u64",
                                },
                            },
                        ],
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 383,
                        end: 398,
                        as_str(): "(u64, u64, u64)",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 341,
            end: 416,
            as_str(): "fn tup_wrapper(a: u64, b: u64, c: u64) -> (u64, u64, u64) {\n    (a, b, c)\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 423,
                            end: 427,
                            as_str(): "TUP2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: FunctionApplication(
                            FunctionApplicationExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 430,
                                                end: 441,
                                                as_str(): "tup_wrapper",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 430,
                                        end: 441,
                                        as_str(): "tup_wrapper",
                                    },
                                },
                                arguments: [
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                2,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 442,
                                            end: 443,
                                            as_str(): "2",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 445,
                                            end: 446,
                                            as_str(): "1",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                21,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 448,
                                            end: 450,
                                            as_str(): "21",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 430,
                            end: 451,
                            as_str(): "tup_wrapper(2, 1, 21)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 417,
                        end: 452,
                        as_str(): "const TUP2 = tup_wrapper(2, 1, 21);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 417,
            end: 452,
            as_str(): "const TUP2 = tup_wrapper(2, 1, 21);",
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
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 457,
                            end: 468,
                            as_str(): "arr_wrapper",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: Return(
                                            Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 519,
                                                                            end: 520,
                                                                            as_str(): "a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 519,
                                                                    end: 520,
                                                                    as_str(): "a",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 522,
                                                                            end: 523,
                                                                            as_str(): "b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 522,
                                                                    end: 523,
                                                                    as_str(): "b",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Variable(
                                                                    BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 525,
                                                                            end: 526,
                                                                            as_str(): "c",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 525,
                                                                    end: 526,
                                                                    as_str(): "c",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 518,
                                                    end: 527,
                                                    as_str(): "[a, b, c]",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 511,
                                            end: 527,
                                            as_str(): "return [a, b, c]",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 511,
                                    end: 527,
                                    as_str(): "return [a, b, c]",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 505,
                            end: 530,
                            as_str(): "{\n    return [a, b, c];\n}",
                        },
                    },
                    parameters: [
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 469,
                                    end: 470,
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 472,
                                end: 475,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 477,
                                    end: 478,
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 480,
                                end: 483,
                                as_str(): "u64",
                            },
                        },
                        FunctionParameter {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 485,
                                    end: 486,
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
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 488,
                                end: 491,
                                as_str(): "u64",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 454,
                        end: 530,
                        as_str(): "fn arr_wrapper(a: u64, b: u64, c: u64) -> [u64; 3] {\n    return [a, b, c];\n}",
                    },
                    return_type: Array(
                        TypeArgument {
                            type_id: TypeId(
                                21,
                            ),
                            initial_type_id: TypeId(
                                21,
                            ),
                            span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 497,
                                end: 500,
                                as_str(): "u64",
                            },
                        },
                        Length {
                            val: 3,
                            span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 502,
                                end: 503,
                                as_str(): "3",
                            },
                        },
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 496,
                        end: 504,
                        as_str(): "[u64; 3]",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 454,
            end: 530,
            as_str(): "fn arr_wrapper(a: u64, b: u64, c: u64) -> [u64; 3] {\n    return [a, b, c];\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 537,
                            end: 541,
                            as_str(): "ARR2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: FunctionApplication(
                            FunctionApplicationExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 544,
                                                end: 555,
                                                as_str(): "arr_wrapper",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 544,
                                        end: 555,
                                        as_str(): "arr_wrapper",
                                    },
                                },
                                arguments: [
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 556,
                                            end: 557,
                                            as_str(): "1",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                2,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 559,
                                            end: 560,
                                            as_str(): "2",
                                        },
                                    },
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                3,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 562,
                                            end: 563,
                                            as_str(): "3",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 544,
                            end: 564,
                            as_str(): "arr_wrapper(1, 2, 3)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 531,
                        end: 565,
                        as_str(): "const ARR2 = arr_wrapper(1, 2, 3);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 531,
            end: 565,
            as_str(): "const ARR2 = arr_wrapper(1, 2, 3);",
        },
    },
    AstNode {
        content: Declaration(
            EnumDeclaration(
                EnumDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 572,
                            end: 575,
                            as_str(): "En1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_parameters: [],
                    variants: [
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 582,
                                    end: 585,
                                    as_str(): "Int",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: UnsignedInteger(
                                SixtyFour,
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 587,
                                end: 590,
                                as_str(): "u64",
                            },
                            tag: 0,
                            span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 582,
                                end: 590,
                                as_str(): "Int: u64",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 596,
                                    end: 599,
                                    as_str(): "Arr",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Array(
                                TypeArgument {
                                    type_id: TypeId(
                                        21,
                                    ),
                                    initial_type_id: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 602,
                                        end: 605,
                                        as_str(): "u64",
                                    },
                                },
                                Length {
                                    val: 3,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 607,
                                        end: 608,
                                        as_str(): "3",
                                    },
                                },
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 601,
                                end: 609,
                                as_str(): "[u64; 3]",
                            },
                            tag: 1,
                            span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 596,
                                end: 609,
                                as_str(): "Arr: [u64; 3]",
                            },
                        },
                        EnumVariant {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 615,
                                    end: 620,
                                    as_str(): "NoVal",
                                },
                                is_raw_ident: false,
                            },
                            attributes: {},
                            type_info: Tuple(
                                [],
                            ),
                            type_span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 622,
                                end: 624,
                                as_str(): "()",
                            },
                            tag: 2,
                            span: Span {
                                src (ptr): 0x00007fb1149f7130,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                ),
                                start: 615,
                                end: 624,
                                as_str(): "NoVal: ()",
                            },
                        },
                    ],
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 567,
                        end: 627,
                        as_str(): "enum En1 {\n    Int: u64,\n    Arr: [u64; 3],\n    NoVal: (),\n}",
                    },
                    visibility: Private,
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 567,
            end: 627,
            as_str(): "enum En1 {\n    Int: u64,\n    Arr: [u64; 3],\n    NoVal: (),\n}",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 635,
                            end: 639,
                            as_str(): "EN1a",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: AmbiguousPathExpression(
                            AmbiguousPathExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: AmbiguousSuffix {
                                            before: TypeBinding {
                                                inner: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 642,
                                                        end: 645,
                                                        as_str(): "En1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 642,
                                                    end: 645,
                                                    as_str(): "En1",
                                                },
                                            },
                                            suffix: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 647,
                                                    end: 650,
                                                    as_str(): "Int",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 642,
                                        end: 650,
                                        as_str(): "En1::Int",
                                    },
                                },
                                args: [
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                101,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 651,
                                            end: 654,
                                            as_str(): "101",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 642,
                            end: 655,
                            as_str(): "En1::Int(101)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 629,
                        end: 656,
                        as_str(): "const EN1a = En1::Int(101);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 629,
            end: 656,
            as_str(): "const EN1a = En1::Int(101);",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 663,
                            end: 667,
                            as_str(): "EN1b",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: AmbiguousPathExpression(
                            AmbiguousPathExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [],
                                        suffix: AmbiguousSuffix {
                                            before: TypeBinding {
                                                inner: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 670,
                                                        end: 673,
                                                        as_str(): "En1",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_arguments: [],
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 670,
                                                    end: 673,
                                                    as_str(): "En1",
                                                },
                                            },
                                            suffix: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 675,
                                                    end: 678,
                                                    as_str(): "Arr",
                                                },
                                                is_raw_ident: false,
                                            },
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 670,
                                        end: 678,
                                        as_str(): "En1::Arr",
                                    },
                                },
                                args: [
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 679,
                                                    end: 683,
                                                    as_str(): "ARR2",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 679,
                                            end: 683,
                                            as_str(): "ARR2",
                                        },
                                    },
                                ],
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 670,
                            end: 684,
                            as_str(): "En1::Arr(ARR2)",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 657,
                        end: 685,
                        as_str(): "const EN1b = En1::Arr(ARR2);",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 657,
            end: 685,
            as_str(): "const EN1b = En1::Arr(ARR2);",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 692,
                            end: 696,
                            as_str(): "EN1c",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: DelineatedPath(
                            DelineatedPathExpression {
                                call_path_binding: TypeBinding {
                                    inner: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 699,
                                                    end: 702,
                                                    as_str(): "En1",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 704,
                                                end: 709,
                                                as_str(): "NoVal",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 704,
                                        end: 709,
                                        as_str(): "NoVal",
                                    },
                                },
                                args: None,
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 699,
                            end: 709,
                            as_str(): "En1::NoVal",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 686,
                        end: 710,
                        as_str(): "const EN1c = En1::NoVal;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 686,
            end: 710,
            as_str(): "const EN1c = En1::NoVal;",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 718,
                            end: 731,
                            as_str(): "ETH_ID0_VALUE",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Subfield(
                            SubfieldExpression {
                                prefix: Expression {
                                    kind: Variable(
                                        BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 734,
                                                end: 741,
                                                as_str(): "ETH_ID0",
                                            },
                                            is_raw_ident: false,
                                        },
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 734,
                                        end: 741,
                                        as_str(): "ETH_ID0",
                                    },
                                },
                                field_to_access: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 742,
                                        end: 747,
                                        as_str(): "value",
                                    },
                                    is_raw_ident: false,
                                },
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 734,
                            end: 747,
                            as_str(): "ETH_ID0.value",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 712,
                        end: 748,
                        as_str(): "const ETH_ID0_VALUE = ETH_ID0.value;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 712,
            end: 748,
            as_str(): "const ETH_ID0_VALUE = ETH_ID0.value;",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 755,
                            end: 764,
                            as_str(): "TUP1_idx2",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: TupleIndex(
                            TupleIndexExpression {
                                prefix: Expression {
                                    kind: Variable(
                                        BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 767,
                                                end: 771,
                                                as_str(): "TUP1",
                                            },
                                            is_raw_ident: false,
                                        },
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fb1149f7130,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                        ),
                                        start: 767,
                                        end: 771,
                                        as_str(): "TUP1",
                                    },
                                },
                                index: 2,
                                index_span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 772,
                                    end: 773,
                                    as_str(): "2",
                                },
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 767,
                            end: 773,
                            as_str(): "TUP1.2",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 749,
                        end: 774,
                        as_str(): "const TUP1_idx2 = TUP1.2;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 749,
            end: 774,
            as_str(): "const TUP1_idx2 = TUP1.2;",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 782,
                            end: 786,
                            as_str(): "INT1",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Literal(
                            Numeric(
                                1,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 789,
                            end: 790,
                            as_str(): "1",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 776,
                        end: 791,
                        as_str(): "const INT1 = 1;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 776,
            end: 791,
            as_str(): "const INT1 = 1;",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 799,
                            end: 808,
                            as_str(): "ZERO_B256",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Literal(
                            B256(
                                [
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
                                    0,
                                    0,
                                ],
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 811,
                            end: 877,
                            as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 793,
                        end: 878,
                        as_str(): "const ZERO_B256 = 0x0000000000000000000000000000000000000000000000000000000000000000;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 793,
            end: 878,
            as_str(): "const ZERO_B256 = 0x0000000000000000000000000000000000000000000000000000000000000000;",
        },
    },
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 885,
                            end: 888,
                            as_str(): "KEY",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: Unknown,
                    type_ascription_span: None,
                    value: Expression {
                        kind: Variable(
                            BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 891,
                                    end: 900,
                                    as_str(): "ZERO_B256",
                                },
                                is_raw_ident: false,
                            },
                        ),
                        span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 891,
                            end: 900,
                            as_str(): "ZERO_B256",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 879,
                        end: 901,
                        as_str(): "const KEY = ZERO_B256;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 879,
            end: 901,
            as_str(): "const KEY = ZERO_B256;",
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
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 906,
                            end: 910,
                            as_str(): "main",
                        },
                        is_raw_ident: false,
                    },
                    visibility: Private,
                    body: CodeBlock {
                        contents: [
                            AstNode {
                                content: Declaration(
                                    ConstantDeclaration(
                                        ConstantDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 932,
                                                    end: 936,
                                                    as_str(): "int1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            attributes: {},
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            value: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        1,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 939,
                                                    end: 940,
                                                    as_str(): "1",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 926,
                                                end: 941,
                                                as_str(): "const int1 = 1;",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 926,
                                    end: 941,
                                    as_str(): "const int1 = 1;",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 946,
                                                                end: 952,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 946,
                                                        end: 952,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: LazyOperator(
                                                            LazyOperatorExpression {
                                                                op: And,
                                                                lhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 958,
                                                                                                    end: 960,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 958,
                                                                                                    end: 960,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 958,
                                                                                                end: 960,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 958,
                                                                                    end: 960,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 953,
                                                                                                end: 957,
                                                                                                as_str(): "int1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 953,
                                                                                        end: 957,
                                                                                        as_str(): "int1",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 961,
                                                                                                end: 965,
                                                                                                as_str(): "INT1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 961,
                                                                                        end: 965,
                                                                                        as_str(): "INT1",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 953,
                                                                        end: 965,
                                                                        as_str(): "int1 == INT1",
                                                                    },
                                                                },
                                                                rhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 979,
                                                                                                    end: 981,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 979,
                                                                                                    end: 981,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 979,
                                                                                                end: 981,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 979,
                                                                                    end: 981,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 969,
                                                                                                end: 978,
                                                                                                as_str(): "ZERO_B256",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 969,
                                                                                        end: 978,
                                                                                        as_str(): "ZERO_B256",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 982,
                                                                                                end: 985,
                                                                                                as_str(): "KEY",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 982,
                                                                                        end: 985,
                                                                                        as_str(): "KEY",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 969,
                                                                        end: 985,
                                                                        as_str(): "ZERO_B256 == KEY",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 953,
                                                            end: 985,
                                                            as_str(): "int1 == INT1 && ZERO_B256 == KEY",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 946,
                                            end: 986,
                                            as_str(): "assert(int1 == INT1 && ZERO_B256 == KEY)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 946,
                                    end: 986,
                                    as_str(): "assert(int1 == INT1 && ZERO_B256 == KEY)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    ConstantDeclaration(
                                        ConstantDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1052,
                                                    end: 1059,
                                                    as_str(): "eth_id0",
                                                },
                                                is_raw_ident: false,
                                            },
                                            attributes: {},
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            value: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1062,
                                                                                end: 1072,
                                                                                as_str(): "ContractId",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1062,
                                                                            end: 1072,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1074,
                                                                            end: 1078,
                                                                            as_str(): "from",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1062,
                                                                end: 1078,
                                                                as_str(): "ContractId::from",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    B256(
                                                                        [
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
                                                                            0,
                                                                            0,
                                                                        ],
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1079,
                                                                    end: 1145,
                                                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000000",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1062,
                                                    end: 1146,
                                                    as_str(): "ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000)",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 1046,
                                                end: 1147,
                                                as_str(): "const eth_id0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1046,
                                    end: 1147,
                                    as_str(): "const eth_id0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    ConstantDeclaration(
                                        ConstantDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1158,
                                                    end: 1165,
                                                    as_str(): "eth_id1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            attributes: {},
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            value: Expression {
                                                kind: AmbiguousPathExpression(
                                                    AmbiguousPathExpression {
                                                        call_path_binding: TypeBinding {
                                                            inner: CallPath {
                                                                prefixes: [],
                                                                suffix: AmbiguousSuffix {
                                                                    before: TypeBinding {
                                                                        inner: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1168,
                                                                                end: 1178,
                                                                                as_str(): "ContractId",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        type_arguments: [],
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1168,
                                                                            end: 1178,
                                                                            as_str(): "ContractId",
                                                                        },
                                                                    },
                                                                    suffix: BaseIdent {
                                                                        name_override_opt: None,
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1180,
                                                                            end: 1184,
                                                                            as_str(): "from",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                },
                                                                is_absolute: false,
                                                            },
                                                            type_arguments: [],
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1168,
                                                                end: 1184,
                                                                as_str(): "ContractId::from",
                                                            },
                                                        },
                                                        args: [
                                                            Expression {
                                                                kind: Literal(
                                                                    B256(
                                                                        [
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
                                                                            0,
                                                                            1,
                                                                        ],
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1185,
                                                                    end: 1251,
                                                                    as_str(): "0x0000000000000000000000000000000000000000000000000000000000000001",
                                                                },
                                                            },
                                                        ],
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1168,
                                                    end: 1252,
                                                    as_str(): "ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001)",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 1152,
                                                end: 1253,
                                                as_str(): "const eth_id1 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001);",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1152,
                                    end: 1253,
                                    as_str(): "const eth_id1 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001);",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1258,
                                                                end: 1264,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 1258,
                                                        end: 1264,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: LazyOperator(
                                                            LazyOperatorExpression {
                                                                op: And,
                                                                lhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1273,
                                                                                                    end: 1275,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1273,
                                                                                                    end: 1275,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1273,
                                                                                                end: 1275,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1273,
                                                                                    end: 1275,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1265,
                                                                                                end: 1272,
                                                                                                as_str(): "eth_id0",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1265,
                                                                                        end: 1272,
                                                                                        as_str(): "eth_id0",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1276,
                                                                                                end: 1283,
                                                                                                as_str(): "ETH_ID0",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1276,
                                                                                        end: 1283,
                                                                                        as_str(): "ETH_ID0",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1265,
                                                                        end: 1283,
                                                                        as_str(): "eth_id0 == ETH_ID0",
                                                                    },
                                                                },
                                                                rhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1295,
                                                                                                    end: 1297,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1295,
                                                                                                    end: 1297,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1295,
                                                                                                end: 1297,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1295,
                                                                                    end: 1297,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1287,
                                                                                                end: 1294,
                                                                                                as_str(): "eth_id1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1287,
                                                                                        end: 1294,
                                                                                        as_str(): "eth_id1",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1298,
                                                                                                end: 1305,
                                                                                                as_str(): "ETH_ID1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1298,
                                                                                        end: 1305,
                                                                                        as_str(): "ETH_ID1",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1287,
                                                                        end: 1305,
                                                                        as_str(): "eth_id1 == ETH_ID1",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1265,
                                                            end: 1305,
                                                            as_str(): "eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 1258,
                                            end: 1306,
                                            as_str(): "assert(eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1258,
                                    end: 1306,
                                    as_str(): "assert(eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    ConstantDeclaration(
                                        ConstantDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1345,
                                                    end: 1347,
                                                    as_str(): "t1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            attributes: {},
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            value: Expression {
                                                kind: Tuple(
                                                    [
                                                        Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    2,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1351,
                                                                end: 1352,
                                                                as_str(): "2",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    1,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1354,
                                                                end: 1355,
                                                                as_str(): "1",
                                                            },
                                                        },
                                                        Expression {
                                                            kind: Literal(
                                                                Numeric(
                                                                    21,
                                                                ),
                                                            ),
                                                            span: Span {
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1357,
                                                                end: 1359,
                                                                as_str(): "21",
                                                            },
                                                        },
                                                    ],
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1350,
                                                    end: 1360,
                                                    as_str(): "(2, 1, 21)",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 1339,
                                                end: 1361,
                                                as_str(): "const t1 = (2, 1, 21);",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1339,
                                    end: 1361,
                                    as_str(): "const t1 = (2, 1, 21);",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1366,
                                                                end: 1372,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 1366,
                                                        end: 1372,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: LazyOperator(
                                                            LazyOperatorExpression {
                                                                op: And,
                                                                lhs: Expression {
                                                                    kind: LazyOperator(
                                                                        LazyOperatorExpression {
                                                                            op: And,
                                                                            lhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1378,
                                                                                                                end: 1380,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1378,
                                                                                                                end: 1380,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1378,
                                                                                                            end: 1380,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1378,
                                                                                                end: 1380,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1373,
                                                                                                                        end: 1375,
                                                                                                                        as_str(): "t1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1373,
                                                                                                                end: 1375,
                                                                                                                as_str(): "t1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 0,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1376,
                                                                                                            end: 1377,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1373,
                                                                                                    end: 1377,
                                                                                                    as_str(): "t1.0",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1381,
                                                                                                                        end: 1385,
                                                                                                                        as_str(): "TUP1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1381,
                                                                                                                end: 1385,
                                                                                                                as_str(): "TUP1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 0,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1386,
                                                                                                            end: 1387,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1381,
                                                                                                    end: 1387,
                                                                                                    as_str(): "TUP1.0",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1373,
                                                                                    end: 1387,
                                                                                    as_str(): "t1.0 == TUP1.0",
                                                                                },
                                                                            },
                                                                            rhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1396,
                                                                                                                end: 1398,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1396,
                                                                                                                end: 1398,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1396,
                                                                                                            end: 1398,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1396,
                                                                                                end: 1398,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1391,
                                                                                                                        end: 1393,
                                                                                                                        as_str(): "t1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1391,
                                                                                                                end: 1393,
                                                                                                                as_str(): "t1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 1,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1394,
                                                                                                            end: 1395,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1391,
                                                                                                    end: 1395,
                                                                                                    as_str(): "t1.1",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1399,
                                                                                                                        end: 1403,
                                                                                                                        as_str(): "TUP1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1399,
                                                                                                                end: 1403,
                                                                                                                as_str(): "TUP1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 1,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1404,
                                                                                                            end: 1405,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1399,
                                                                                                    end: 1405,
                                                                                                    as_str(): "TUP1.1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1391,
                                                                                    end: 1405,
                                                                                    as_str(): "t1.1 == TUP1.1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1373,
                                                                        end: 1405,
                                                                        as_str(): "t1.0 == TUP1.0 && t1.1 == TUP1.1",
                                                                    },
                                                                },
                                                                rhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1414,
                                                                                                    end: 1416,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1414,
                                                                                                    end: 1416,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1414,
                                                                                                end: 1416,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1414,
                                                                                    end: 1416,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: TupleIndex(
                                                                                        TupleIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1409,
                                                                                                            end: 1411,
                                                                                                            as_str(): "t1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1409,
                                                                                                    end: 1411,
                                                                                                    as_str(): "t1",
                                                                                                },
                                                                                            },
                                                                                            index: 2,
                                                                                            index_span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1412,
                                                                                                end: 1413,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1409,
                                                                                        end: 1413,
                                                                                        as_str(): "t1.2",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: TupleIndex(
                                                                                        TupleIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1417,
                                                                                                            end: 1421,
                                                                                                            as_str(): "TUP1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1417,
                                                                                                    end: 1421,
                                                                                                    as_str(): "TUP1",
                                                                                                },
                                                                                            },
                                                                                            index: 2,
                                                                                            index_span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1422,
                                                                                                end: 1423,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1417,
                                                                                        end: 1423,
                                                                                        as_str(): "TUP1.2",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1409,
                                                                        end: 1423,
                                                                        as_str(): "t1.2 == TUP1.2",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1373,
                                                            end: 1423,
                                                            as_str(): "t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 1366,
                                            end: 1424,
                                            as_str(): "assert(t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1366,
                                    end: 1424,
                                    as_str(): "assert(t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2)",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1430,
                                                                end: 1436,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 1430,
                                                        end: 1436,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: LazyOperator(
                                                            LazyOperatorExpression {
                                                                op: And,
                                                                lhs: Expression {
                                                                    kind: LazyOperator(
                                                                        LazyOperatorExpression {
                                                                            op: And,
                                                                            lhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1442,
                                                                                                                end: 1444,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1442,
                                                                                                                end: 1444,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1442,
                                                                                                            end: 1444,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1442,
                                                                                                end: 1444,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1437,
                                                                                                                        end: 1439,
                                                                                                                        as_str(): "t1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1437,
                                                                                                                end: 1439,
                                                                                                                as_str(): "t1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 0,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1440,
                                                                                                            end: 1441,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1437,
                                                                                                    end: 1441,
                                                                                                    as_str(): "t1.0",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1445,
                                                                                                                        end: 1449,
                                                                                                                        as_str(): "TUP2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1445,
                                                                                                                end: 1449,
                                                                                                                as_str(): "TUP2",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 0,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1450,
                                                                                                            end: 1451,
                                                                                                            as_str(): "0",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1445,
                                                                                                    end: 1451,
                                                                                                    as_str(): "TUP2.0",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1437,
                                                                                    end: 1451,
                                                                                    as_str(): "t1.0 == TUP2.0",
                                                                                },
                                                                            },
                                                                            rhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1460,
                                                                                                                end: 1462,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1460,
                                                                                                                end: 1462,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1460,
                                                                                                            end: 1462,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1460,
                                                                                                end: 1462,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1455,
                                                                                                                        end: 1457,
                                                                                                                        as_str(): "t1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1455,
                                                                                                                end: 1457,
                                                                                                                as_str(): "t1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 1,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1458,
                                                                                                            end: 1459,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1455,
                                                                                                    end: 1459,
                                                                                                    as_str(): "t1.1",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: TupleIndex(
                                                                                                    TupleIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1463,
                                                                                                                        end: 1467,
                                                                                                                        as_str(): "TUP2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1463,
                                                                                                                end: 1467,
                                                                                                                as_str(): "TUP2",
                                                                                                            },
                                                                                                        },
                                                                                                        index: 1,
                                                                                                        index_span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1468,
                                                                                                            end: 1469,
                                                                                                            as_str(): "1",
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1463,
                                                                                                    end: 1469,
                                                                                                    as_str(): "TUP2.1",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1455,
                                                                                    end: 1469,
                                                                                    as_str(): "t1.1 == TUP2.1",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1437,
                                                                        end: 1469,
                                                                        as_str(): "t1.0 == TUP2.0 && t1.1 == TUP2.1",
                                                                    },
                                                                },
                                                                rhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1478,
                                                                                                    end: 1480,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1478,
                                                                                                    end: 1480,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1478,
                                                                                                end: 1480,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1478,
                                                                                    end: 1480,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: TupleIndex(
                                                                                        TupleIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1473,
                                                                                                            end: 1475,
                                                                                                            as_str(): "t1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1473,
                                                                                                    end: 1475,
                                                                                                    as_str(): "t1",
                                                                                                },
                                                                                            },
                                                                                            index: 2,
                                                                                            index_span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1476,
                                                                                                end: 1477,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1473,
                                                                                        end: 1477,
                                                                                        as_str(): "t1.2",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: TupleIndex(
                                                                                        TupleIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1481,
                                                                                                            end: 1485,
                                                                                                            as_str(): "TUP2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1481,
                                                                                                    end: 1485,
                                                                                                    as_str(): "TUP2",
                                                                                                },
                                                                                            },
                                                                                            index: 2,
                                                                                            index_span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1486,
                                                                                                end: 1487,
                                                                                                as_str(): "2",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1481,
                                                                                        end: 1487,
                                                                                        as_str(): "TUP2.2",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1473,
                                                                        end: 1487,
                                                                        as_str(): "t1.2 == TUP2.2",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1437,
                                                            end: 1487,
                                                            as_str(): "t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 1430,
                                            end: 1488,
                                            as_str(): "assert(t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1430,
                                    end: 1488,
                                    as_str(): "assert(t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2)",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    ConstantDeclaration(
                                        ConstantDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1500,
                                                    end: 1502,
                                                    as_str(): "a1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            attributes: {},
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            value: Expression {
                                                kind: Array(
                                                    ArrayExpression {
                                                        contents: [
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        1,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1506,
                                                                    end: 1507,
                                                                    as_str(): "1",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        2,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1509,
                                                                    end: 1510,
                                                                    as_str(): "2",
                                                                },
                                                            },
                                                            Expression {
                                                                kind: Literal(
                                                                    Numeric(
                                                                        3,
                                                                    ),
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1512,
                                                                    end: 1513,
                                                                    as_str(): "3",
                                                                },
                                                            },
                                                        ],
                                                        length_span: None,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1505,
                                                    end: 1514,
                                                    as_str(): "[1, 2, 3]",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb1149f7130,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                ),
                                                start: 1494,
                                                end: 1515,
                                                as_str(): "const a1 = [1, 2, 3];",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1494,
                                    end: 1515,
                                    as_str(): "const a1 = [1, 2, 3];",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1520,
                                                                end: 1526,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 1520,
                                                        end: 1526,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: LazyOperator(
                                                            LazyOperatorExpression {
                                                                op: And,
                                                                lhs: Expression {
                                                                    kind: LazyOperator(
                                                                        LazyOperatorExpression {
                                                                            op: And,
                                                                            lhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1533,
                                                                                                                end: 1535,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1533,
                                                                                                                end: 1535,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1533,
                                                                                                            end: 1535,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1533,
                                                                                                end: 1535,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1527,
                                                                                                                        end: 1529,
                                                                                                                        as_str(): "a1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1527,
                                                                                                                end: 1529,
                                                                                                                as_str(): "a1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    0,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1530,
                                                                                                                end: 1531,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1527,
                                                                                                    end: 1532,
                                                                                                    as_str(): "a1[0]",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1536,
                                                                                                                        end: 1540,
                                                                                                                        as_str(): "ARR1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1536,
                                                                                                                end: 1540,
                                                                                                                as_str(): "ARR1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    0,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1541,
                                                                                                                end: 1542,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1536,
                                                                                                    end: 1543,
                                                                                                    as_str(): "ARR1[0]",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1527,
                                                                                    end: 1543,
                                                                                    as_str(): "a1[0] == ARR1[0]",
                                                                                },
                                                                            },
                                                                            rhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1553,
                                                                                                                end: 1555,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1553,
                                                                                                                end: 1555,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1553,
                                                                                                            end: 1555,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1553,
                                                                                                end: 1555,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1547,
                                                                                                                        end: 1549,
                                                                                                                        as_str(): "a1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1547,
                                                                                                                end: 1549,
                                                                                                                as_str(): "a1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    1,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1550,
                                                                                                                end: 1551,
                                                                                                                as_str(): "1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1547,
                                                                                                    end: 1552,
                                                                                                    as_str(): "a1[1]",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1556,
                                                                                                                        end: 1560,
                                                                                                                        as_str(): "ARR1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1556,
                                                                                                                end: 1560,
                                                                                                                as_str(): "ARR1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    1,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1561,
                                                                                                                end: 1562,
                                                                                                                as_str(): "1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1556,
                                                                                                    end: 1563,
                                                                                                    as_str(): "ARR1[1]",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1547,
                                                                                    end: 1563,
                                                                                    as_str(): "a1[1] == ARR1[1]",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1527,
                                                                        end: 1563,
                                                                        as_str(): "a1[0] == ARR1[0] && a1[1] == ARR1[1]",
                                                                    },
                                                                },
                                                                rhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1573,
                                                                                                    end: 1575,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1573,
                                                                                                    end: 1575,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1573,
                                                                                                end: 1575,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1573,
                                                                                    end: 1575,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: ArrayIndex(
                                                                                        ArrayIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1567,
                                                                                                            end: 1569,
                                                                                                            as_str(): "a1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1567,
                                                                                                    end: 1569,
                                                                                                    as_str(): "a1",
                                                                                                },
                                                                                            },
                                                                                            index: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        2,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1570,
                                                                                                    end: 1571,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1567,
                                                                                        end: 1572,
                                                                                        as_str(): "a1[2]",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: ArrayIndex(
                                                                                        ArrayIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1576,
                                                                                                            end: 1580,
                                                                                                            as_str(): "ARR1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1576,
                                                                                                    end: 1580,
                                                                                                    as_str(): "ARR1",
                                                                                                },
                                                                                            },
                                                                                            index: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        2,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1581,
                                                                                                    end: 1582,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1576,
                                                                                        end: 1583,
                                                                                        as_str(): "ARR1[2]",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1567,
                                                                        end: 1583,
                                                                        as_str(): "a1[2] == ARR1[2]",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1527,
                                                            end: 1583,
                                                            as_str(): "a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 1520,
                                            end: 1584,
                                            as_str(): "assert(a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1520,
                                    end: 1584,
                                    as_str(): "assert(a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2])",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 1590,
                                                                end: 1596,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 1590,
                                                        end: 1596,
                                                        as_str(): "assert",
                                                    },
                                                },
                                                arguments: [
                                                    Expression {
                                                        kind: LazyOperator(
                                                            LazyOperatorExpression {
                                                                op: And,
                                                                lhs: Expression {
                                                                    kind: LazyOperator(
                                                                        LazyOperatorExpression {
                                                                            op: And,
                                                                            lhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1603,
                                                                                                                end: 1605,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1603,
                                                                                                                end: 1605,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1603,
                                                                                                            end: 1605,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1603,
                                                                                                end: 1605,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1597,
                                                                                                                        end: 1599,
                                                                                                                        as_str(): "a1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1597,
                                                                                                                end: 1599,
                                                                                                                as_str(): "a1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    0,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1600,
                                                                                                                end: 1601,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1597,
                                                                                                    end: 1602,
                                                                                                    as_str(): "a1[0]",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1606,
                                                                                                                        end: 1610,
                                                                                                                        as_str(): "ARR2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1606,
                                                                                                                end: 1610,
                                                                                                                as_str(): "ARR2",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    0,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1611,
                                                                                                                end: 1612,
                                                                                                                as_str(): "0",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1606,
                                                                                                    end: 1613,
                                                                                                    as_str(): "ARR2[0]",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1597,
                                                                                    end: 1613,
                                                                                    as_str(): "a1[0] == ARR2[0]",
                                                                                },
                                                                            },
                                                                            rhs: Expression {
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
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1623,
                                                                                                                end: 1625,
                                                                                                                as_str(): "==",
                                                                                                            },
                                                                                                            is_raw_ident: false,
                                                                                                        },
                                                                                                        BaseIdent {
                                                                                                            name_override_opt: Some(
                                                                                                                "ops",
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1623,
                                                                                                                end: 1625,
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
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1623,
                                                                                                            end: 1625,
                                                                                                            as_str(): "==",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: true,
                                                                                                },
                                                                                            },
                                                                                            type_arguments: [],
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1623,
                                                                                                end: 1625,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                        },
                                                                                        contract_call_params: [],
                                                                                        arguments: [
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1617,
                                                                                                                        end: 1619,
                                                                                                                        as_str(): "a1",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1617,
                                                                                                                end: 1619,
                                                                                                                as_str(): "a1",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    1,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1620,
                                                                                                                end: 1621,
                                                                                                                as_str(): "1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1617,
                                                                                                    end: 1622,
                                                                                                    as_str(): "a1[1]",
                                                                                                },
                                                                                            },
                                                                                            Expression {
                                                                                                kind: ArrayIndex(
                                                                                                    ArrayIndexExpression {
                                                                                                        prefix: Expression {
                                                                                                            kind: Variable(
                                                                                                                BaseIdent {
                                                                                                                    name_override_opt: None,
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1626,
                                                                                                                        end: 1630,
                                                                                                                        as_str(): "ARR2",
                                                                                                                    },
                                                                                                                    is_raw_ident: false,
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1626,
                                                                                                                end: 1630,
                                                                                                                as_str(): "ARR2",
                                                                                                            },
                                                                                                        },
                                                                                                        index: Expression {
                                                                                                            kind: Literal(
                                                                                                                Numeric(
                                                                                                                    1,
                                                                                                                ),
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1631,
                                                                                                                end: 1632,
                                                                                                                as_str(): "1",
                                                                                                            },
                                                                                                        },
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1626,
                                                                                                    end: 1633,
                                                                                                    as_str(): "ARR2[1]",
                                                                                                },
                                                                                            },
                                                                                        ],
                                                                                    },
                                                                                ),
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1617,
                                                                                    end: 1633,
                                                                                    as_str(): "a1[1] == ARR2[1]",
                                                                                },
                                                                            },
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1597,
                                                                        end: 1633,
                                                                        as_str(): "a1[0] == ARR2[0] && a1[1] == ARR2[1]",
                                                                    },
                                                                },
                                                                rhs: Expression {
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
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1643,
                                                                                                    end: 1645,
                                                                                                    as_str(): "==",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                            BaseIdent {
                                                                                                name_override_opt: Some(
                                                                                                    "ops",
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1643,
                                                                                                    end: 1645,
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
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1643,
                                                                                                end: 1645,
                                                                                                as_str(): "==",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: true,
                                                                                    },
                                                                                },
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1643,
                                                                                    end: 1645,
                                                                                    as_str(): "==",
                                                                                },
                                                                            },
                                                                            contract_call_params: [],
                                                                            arguments: [
                                                                                Expression {
                                                                                    kind: ArrayIndex(
                                                                                        ArrayIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1637,
                                                                                                            end: 1639,
                                                                                                            as_str(): "a1",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1637,
                                                                                                    end: 1639,
                                                                                                    as_str(): "a1",
                                                                                                },
                                                                                            },
                                                                                            index: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        2,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1640,
                                                                                                    end: 1641,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1637,
                                                                                        end: 1642,
                                                                                        as_str(): "a1[2]",
                                                                                    },
                                                                                },
                                                                                Expression {
                                                                                    kind: ArrayIndex(
                                                                                        ArrayIndexExpression {
                                                                                            prefix: Expression {
                                                                                                kind: Variable(
                                                                                                    BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1646,
                                                                                                            end: 1650,
                                                                                                            as_str(): "ARR2",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1646,
                                                                                                    end: 1650,
                                                                                                    as_str(): "ARR2",
                                                                                                },
                                                                                            },
                                                                                            index: Expression {
                                                                                                kind: Literal(
                                                                                                    Numeric(
                                                                                                        2,
                                                                                                    ),
                                                                                                ),
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1651,
                                                                                                    end: 1652,
                                                                                                    as_str(): "2",
                                                                                                },
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1646,
                                                                                        end: 1653,
                                                                                        as_str(): "ARR2[2]",
                                                                                    },
                                                                                },
                                                                            ],
                                                                        },
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 1637,
                                                                        end: 1653,
                                                                        as_str(): "a1[2] == ARR2[2]",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1597,
                                                            end: 1653,
                                                            as_str(): "a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2]",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 1590,
                                            end: 1654,
                                            as_str(): "assert(a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2])",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1590,
                                    end: 1654,
                                    as_str(): "assert(a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2])",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_1",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1679,
                                                                            end: 1683,
                                                                            as_str(): "EN1a",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1679,
                                                                                    end: 1683,
                                                                                    as_str(): "EN1a",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1679,
                                                                            end: 1683,
                                                                            as_str(): "EN1a",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1673,
                                                            end: 1807,
                                                            as_str(): "match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_1",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1679,
                                                                                        end: 1683,
                                                                                        as_str(): "EN1a",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1679,
                                                                                end: 1683,
                                                                                as_str(): "EN1a",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1694,
                                                                                                    end: 1697,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1699,
                                                                                                end: 1702,
                                                                                                as_str(): "Int",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1703,
                                                                                                end: 1704,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1703,
                                                                                            end: 1704,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1694,
                                                                                        end: 1705,
                                                                                        as_str(): "En1::Int(i)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1709,
                                                                                                            end: 1715,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1709,
                                                                                                    end: 1715,
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
                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1718,
                                                                                                                                    end: 1720,
                                                                                                                                    as_str(): "==",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            BaseIdent {
                                                                                                                                name_override_opt: Some(
                                                                                                                                    "ops",
                                                                                                                                ),
                                                                                                                                span: Span {
                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1718,
                                                                                                                                    end: 1720,
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
                                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1718,
                                                                                                                                end: 1720,
                                                                                                                                as_str(): "==",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                        is_absolute: true,
                                                                                                                    },
                                                                                                                },
                                                                                                                type_arguments: [],
                                                                                                                span: Span {
                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                    path: Some(
                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                    ),
                                                                                                                    start: 1718,
                                                                                                                    end: 1720,
                                                                                                                    as_str(): "==",
                                                                                                                },
                                                                                                            },
                                                                                                            contract_call_params: [],
                                                                                                            arguments: [
                                                                                                                Expression {
                                                                                                                    kind: Variable(
                                                                                                                        BaseIdent {
                                                                                                                            name_override_opt: None,
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1716,
                                                                                                                                end: 1717,
                                                                                                                                as_str(): "i",
                                                                                                                            },
                                                                                                                            is_raw_ident: false,
                                                                                                                        },
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1716,
                                                                                                                        end: 1717,
                                                                                                                        as_str(): "i",
                                                                                                                    },
                                                                                                                },
                                                                                                                Expression {
                                                                                                                    kind: Literal(
                                                                                                                        Numeric(
                                                                                                                            101,
                                                                                                                        ),
                                                                                                                    ),
                                                                                                                    span: Span {
                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                        path: Some(
                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                        ),
                                                                                                                        start: 1721,
                                                                                                                        end: 1724,
                                                                                                                        as_str(): "101",
                                                                                                                    },
                                                                                                                },
                                                                                                            ],
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1716,
                                                                                                        end: 1724,
                                                                                                        as_str(): "i == 101",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1709,
                                                                                        end: 1725,
                                                                                        as_str(): "assert(i == 101)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1694,
                                                                                    end: 1726,
                                                                                    as_str(): "En1::Int(i) => assert(i == 101),",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1735,
                                                                                                    end: 1738,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1740,
                                                                                                end: 1743,
                                                                                                as_str(): "Arr",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: CatchAll {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1744,
                                                                                            end: 1745,
                                                                                            as_str(): "_",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1735,
                                                                                        end: 1746,
                                                                                        as_str(): "En1::Arr(_)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1750,
                                                                                                            end: 1756,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1750,
                                                                                                    end: 1756,
                                                                                                    as_str(): "assert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            false,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1757,
                                                                                                        end: 1762,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1750,
                                                                                        end: 1763,
                                                                                        as_str(): "assert(false)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1735,
                                                                                    end: 1764,
                                                                                    as_str(): "En1::Arr(_) => assert(false),",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1773,
                                                                                                    end: 1776,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1778,
                                                                                                end: 1783,
                                                                                                as_str(): "NoVal",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: CatchAll {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1773,
                                                                                            end: 1783,
                                                                                            as_str(): "En1::NoVal",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1773,
                                                                                        end: 1783,
                                                                                        as_str(): "En1::NoVal",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1787,
                                                                                                            end: 1793,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1787,
                                                                                                    end: 1793,
                                                                                                    as_str(): "assert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            false,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1794,
                                                                                                        end: 1799,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1787,
                                                                                        end: 1800,
                                                                                        as_str(): "assert(false)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1773,
                                                                                    end: 1801,
                                                                                    as_str(): "En1::NoVal => assert(false),",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1673,
                                                                    end: 1807,
                                                                    as_str(): "match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1673,
                                                            end: 1807,
                                                            as_str(): "match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1673,
                                                    end: 1807,
                                                    as_str(): "match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 1673,
                                            end: 1807,
                                            as_str(): "match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1673,
                                    end: 1807,
                                    as_str(): "match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_2",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1818,
                                                                            end: 1822,
                                                                            as_str(): "EN1b",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1818,
                                                                                    end: 1822,
                                                                                    as_str(): "EN1b",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 1818,
                                                                            end: 1822,
                                                                            as_str(): "EN1b",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1812,
                                                            end: 2023,
                                                            as_str(): "match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_2",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1818,
                                                                                        end: 1822,
                                                                                        as_str(): "EN1b",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 1818,
                                                                                end: 1822,
                                                                                as_str(): "EN1b",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1833,
                                                                                                    end: 1836,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1838,
                                                                                                end: 1841,
                                                                                                as_str(): "Int",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1842,
                                                                                                end: 1843,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1842,
                                                                                            end: 1843,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1833,
                                                                                        end: 1844,
                                                                                        as_str(): "En1::Int(i)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 1848,
                                                                                                            end: 1854,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1848,
                                                                                                    end: 1854,
                                                                                                    as_str(): "assert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            false,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1855,
                                                                                                        end: 1860,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1848,
                                                                                        end: 1861,
                                                                                        as_str(): "assert(false)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1833,
                                                                                    end: 1862,
                                                                                    as_str(): "En1::Int(i) => assert(false),",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1871,
                                                                                                    end: 1874,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1876,
                                                                                                end: 1879,
                                                                                                as_str(): "Arr",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1880,
                                                                                                end: 1883,
                                                                                                as_str(): "arr",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1880,
                                                                                            end: 1883,
                                                                                            as_str(): "arr",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1871,
                                                                                        end: 1884,
                                                                                        as_str(): "En1::Arr(arr)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: CodeBlock(
                                                                                        CodeBlock {
                                                                                            contents: [
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
                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                    path: Some(
                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                    ),
                                                                                                                                    start: 1902,
                                                                                                                                    end: 1908,
                                                                                                                                    as_str(): "assert",
                                                                                                                                },
                                                                                                                                is_raw_ident: false,
                                                                                                                            },
                                                                                                                            is_absolute: false,
                                                                                                                        },
                                                                                                                        type_arguments: [],
                                                                                                                        span: Span {
                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                            path: Some(
                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                            ),
                                                                                                                            start: 1902,
                                                                                                                            end: 1908,
                                                                                                                            as_str(): "assert",
                                                                                                                        },
                                                                                                                    },
                                                                                                                    arguments: [
                                                                                                                        Expression {
                                                                                                                            kind: LazyOperator(
                                                                                                                                LazyOperatorExpression {
                                                                                                                                    op: And,
                                                                                                                                    lhs: Expression {
                                                                                                                                        kind: LazyOperator(
                                                                                                                                            LazyOperatorExpression {
                                                                                                                                                op: And,
                                                                                                                                                lhs: Expression {
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
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1916,
                                                                                                                                                                                    end: 1918,
                                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                                },
                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                            },
                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                                    "ops",
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1916,
                                                                                                                                                                                    end: 1918,
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
                                                                                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 1916,
                                                                                                                                                                                end: 1918,
                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        is_absolute: true,
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                type_arguments: [],
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 1916,
                                                                                                                                                                    end: 1918,
                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            contract_call_params: [],
                                                                                                                                                            arguments: [
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: ArrayIndex(
                                                                                                                                                                        ArrayIndexExpression {
                                                                                                                                                                            prefix: Expression {
                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 1909,
                                                                                                                                                                                            end: 1912,
                                                                                                                                                                                            as_str(): "arr",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1909,
                                                                                                                                                                                    end: 1912,
                                                                                                                                                                                    as_str(): "arr",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            index: Expression {
                                                                                                                                                                                kind: Literal(
                                                                                                                                                                                    Numeric(
                                                                                                                                                                                        0,
                                                                                                                                                                                    ),
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1913,
                                                                                                                                                                                    end: 1914,
                                                                                                                                                                                    as_str(): "0",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1909,
                                                                                                                                                                        end: 1915,
                                                                                                                                                                        as_str(): "arr[0]",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: ArrayIndex(
                                                                                                                                                                        ArrayIndexExpression {
                                                                                                                                                                            prefix: Expression {
                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 1919,
                                                                                                                                                                                            end: 1923,
                                                                                                                                                                                            as_str(): "ARR1",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1919,
                                                                                                                                                                                    end: 1923,
                                                                                                                                                                                    as_str(): "ARR1",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            index: Expression {
                                                                                                                                                                                kind: Literal(
                                                                                                                                                                                    Numeric(
                                                                                                                                                                                        0,
                                                                                                                                                                                    ),
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1924,
                                                                                                                                                                                    end: 1925,
                                                                                                                                                                                    as_str(): "0",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1919,
                                                                                                                                                                        end: 1926,
                                                                                                                                                                        as_str(): "ARR1[0]",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1909,
                                                                                                                                                        end: 1926,
                                                                                                                                                        as_str(): "arr[0] == ARR1[0]",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                rhs: Expression {
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
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1937,
                                                                                                                                                                                    end: 1939,
                                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                                },
                                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                                            },
                                                                                                                                                                            BaseIdent {
                                                                                                                                                                                name_override_opt: Some(
                                                                                                                                                                                    "ops",
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1937,
                                                                                                                                                                                    end: 1939,
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
                                                                                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 1937,
                                                                                                                                                                                end: 1939,
                                                                                                                                                                                as_str(): "==",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                        is_absolute: true,
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                type_arguments: [],
                                                                                                                                                                span: Span {
                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 1937,
                                                                                                                                                                    end: 1939,
                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                            contract_call_params: [],
                                                                                                                                                            arguments: [
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: ArrayIndex(
                                                                                                                                                                        ArrayIndexExpression {
                                                                                                                                                                            prefix: Expression {
                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 1930,
                                                                                                                                                                                            end: 1933,
                                                                                                                                                                                            as_str(): "arr",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1930,
                                                                                                                                                                                    end: 1933,
                                                                                                                                                                                    as_str(): "arr",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            index: Expression {
                                                                                                                                                                                kind: Literal(
                                                                                                                                                                                    Numeric(
                                                                                                                                                                                        1,
                                                                                                                                                                                    ),
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1934,
                                                                                                                                                                                    end: 1935,
                                                                                                                                                                                    as_str(): "1",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1930,
                                                                                                                                                                        end: 1936,
                                                                                                                                                                        as_str(): "arr[1]",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                Expression {
                                                                                                                                                                    kind: ArrayIndex(
                                                                                                                                                                        ArrayIndexExpression {
                                                                                                                                                                            prefix: Expression {
                                                                                                                                                                                kind: Variable(
                                                                                                                                                                                    BaseIdent {
                                                                                                                                                                                        name_override_opt: None,
                                                                                                                                                                                        span: Span {
                                                                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                            path: Some(
                                                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                            ),
                                                                                                                                                                                            start: 1940,
                                                                                                                                                                                            end: 1944,
                                                                                                                                                                                            as_str(): "ARR1",
                                                                                                                                                                                        },
                                                                                                                                                                                        is_raw_ident: false,
                                                                                                                                                                                    },
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1940,
                                                                                                                                                                                    end: 1944,
                                                                                                                                                                                    as_str(): "ARR1",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                            index: Expression {
                                                                                                                                                                                kind: Literal(
                                                                                                                                                                                    Numeric(
                                                                                                                                                                                        1,
                                                                                                                                                                                    ),
                                                                                                                                                                                ),
                                                                                                                                                                                span: Span {
                                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                    path: Some(
                                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                    ),
                                                                                                                                                                                    start: 1945,
                                                                                                                                                                                    end: 1946,
                                                                                                                                                                                    as_str(): "1",
                                                                                                                                                                                },
                                                                                                                                                                            },
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1940,
                                                                                                                                                                        end: 1947,
                                                                                                                                                                        as_str(): "ARR1[1]",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            ],
                                                                                                                                                        },
                                                                                                                                                    ),
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1930,
                                                                                                                                                        end: 1947,
                                                                                                                                                        as_str(): "arr[1] == ARR1[1]",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1909,
                                                                                                                                            end: 1947,
                                                                                                                                            as_str(): "arr[0] == ARR1[0] && arr[1] == ARR1[1]",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                    rhs: Expression {
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
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1958,
                                                                                                                                                                        end: 1960,
                                                                                                                                                                        as_str(): "==",
                                                                                                                                                                    },
                                                                                                                                                                    is_raw_ident: false,
                                                                                                                                                                },
                                                                                                                                                                BaseIdent {
                                                                                                                                                                    name_override_opt: Some(
                                                                                                                                                                        "ops",
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1958,
                                                                                                                                                                        end: 1960,
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
                                                                                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                    path: Some(
                                                                                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                    ),
                                                                                                                                                                    start: 1958,
                                                                                                                                                                    end: 1960,
                                                                                                                                                                    as_str(): "==",
                                                                                                                                                                },
                                                                                                                                                                is_raw_ident: false,
                                                                                                                                                            },
                                                                                                                                                            is_absolute: true,
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    type_arguments: [],
                                                                                                                                                    span: Span {
                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                        path: Some(
                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                        ),
                                                                                                                                                        start: 1958,
                                                                                                                                                        end: 1960,
                                                                                                                                                        as_str(): "==",
                                                                                                                                                    },
                                                                                                                                                },
                                                                                                                                                contract_call_params: [],
                                                                                                                                                arguments: [
                                                                                                                                                    Expression {
                                                                                                                                                        kind: ArrayIndex(
                                                                                                                                                            ArrayIndexExpression {
                                                                                                                                                                prefix: Expression {
                                                                                                                                                                    kind: Variable(
                                                                                                                                                                        BaseIdent {
                                                                                                                                                                            name_override_opt: None,
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 1951,
                                                                                                                                                                                end: 1954,
                                                                                                                                                                                as_str(): "arr",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1951,
                                                                                                                                                                        end: 1954,
                                                                                                                                                                        as_str(): "arr",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                index: Expression {
                                                                                                                                                                    kind: Literal(
                                                                                                                                                                        Numeric(
                                                                                                                                                                            2,
                                                                                                                                                                        ),
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1955,
                                                                                                                                                                        end: 1956,
                                                                                                                                                                        as_str(): "2",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1951,
                                                                                                                                                            end: 1957,
                                                                                                                                                            as_str(): "arr[2]",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                    Expression {
                                                                                                                                                        kind: ArrayIndex(
                                                                                                                                                            ArrayIndexExpression {
                                                                                                                                                                prefix: Expression {
                                                                                                                                                                    kind: Variable(
                                                                                                                                                                        BaseIdent {
                                                                                                                                                                            name_override_opt: None,
                                                                                                                                                                            span: Span {
                                                                                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                                path: Some(
                                                                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                                ),
                                                                                                                                                                                start: 1961,
                                                                                                                                                                                end: 1965,
                                                                                                                                                                                as_str(): "ARR1",
                                                                                                                                                                            },
                                                                                                                                                                            is_raw_ident: false,
                                                                                                                                                                        },
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1961,
                                                                                                                                                                        end: 1965,
                                                                                                                                                                        as_str(): "ARR1",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                                index: Expression {
                                                                                                                                                                    kind: Literal(
                                                                                                                                                                        Numeric(
                                                                                                                                                                            2,
                                                                                                                                                                        ),
                                                                                                                                                                    ),
                                                                                                                                                                    span: Span {
                                                                                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                                                                                        path: Some(
                                                                                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                                        ),
                                                                                                                                                                        start: 1966,
                                                                                                                                                                        end: 1967,
                                                                                                                                                                        as_str(): "2",
                                                                                                                                                                    },
                                                                                                                                                                },
                                                                                                                                                            },
                                                                                                                                                        ),
                                                                                                                                                        span: Span {
                                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                                            path: Some(
                                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                                            ),
                                                                                                                                                            start: 1961,
                                                                                                                                                            end: 1968,
                                                                                                                                                            as_str(): "ARR1[2]",
                                                                                                                                                        },
                                                                                                                                                    },
                                                                                                                                                ],
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                        span: Span {
                                                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                                                            path: Some(
                                                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                            ),
                                                                                                                                            start: 1951,
                                                                                                                                            end: 1968,
                                                                                                                                            as_str(): "arr[2] == ARR1[2]",
                                                                                                                                        },
                                                                                                                                    },
                                                                                                                                },
                                                                                                                            ),
                                                                                                                            span: Span {
                                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                                path: Some(
                                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                                ),
                                                                                                                                start: 1909,
                                                                                                                                end: 1968,
                                                                                                                                as_str(): "arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]",
                                                                                                                            },
                                                                                                                        },
                                                                                                                    ],
                                                                                                                },
                                                                                                            ),
                                                                                                            span: Span {
                                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                                path: Some(
                                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                                ),
                                                                                                                start: 1902,
                                                                                                                end: 1969,
                                                                                                                as_str(): "assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2])",
                                                                                                            },
                                                                                                        },
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 1902,
                                                                                                        end: 1969,
                                                                                                        as_str(): "assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2])",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                            whole_block_span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1888,
                                                                                                end: 1980,
                                                                                                as_str(): "{\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }",
                                                                                            },
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1888,
                                                                                        end: 1980,
                                                                                        as_str(): "{\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1871,
                                                                                    end: 1980,
                                                                                    as_str(): "En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 1989,
                                                                                                    end: 1992,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 1994,
                                                                                                end: 1999,
                                                                                                as_str(): "NoVal",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: CatchAll {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 1989,
                                                                                            end: 1999,
                                                                                            as_str(): "En1::NoVal",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 1989,
                                                                                        end: 1999,
                                                                                        as_str(): "En1::NoVal",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2003,
                                                                                                            end: 2009,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2003,
                                                                                                    end: 2009,
                                                                                                    as_str(): "assert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            false,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2010,
                                                                                                        end: 2015,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2003,
                                                                                        end: 2016,
                                                                                        as_str(): "assert(false)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 1989,
                                                                                    end: 2017,
                                                                                    as_str(): "En1::NoVal => assert(false),",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 1812,
                                                                    end: 2023,
                                                                    as_str(): "match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 1812,
                                                            end: 2023,
                                                            as_str(): "match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 1812,
                                                    end: 2023,
                                                    as_str(): "match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 1812,
                                            end: 2023,
                                            as_str(): "match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 1812,
                                    end: 2023,
                                    as_str(): "match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }",
                                },
                            },
                            AstNode {
                                content: Expression(
                                    Expression {
                                        kind: CodeBlock(
                                            CodeBlock {
                                                contents: [
                                                    AstNode {
                                                        content: Declaration(
                                                            VariableDeclaration(
                                                                VariableDeclaration {
                                                                    name: BaseIdent {
                                                                        name_override_opt: Some(
                                                                            "__match_return_var_name_3",
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2034,
                                                                            end: 2038,
                                                                            as_str(): "EN1c",
                                                                        },
                                                                        is_raw_ident: false,
                                                                    },
                                                                    type_ascription: Unknown,
                                                                    type_ascription_span: None,
                                                                    body: Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2034,
                                                                                    end: 2038,
                                                                                    as_str(): "EN1c",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2034,
                                                                            end: 2038,
                                                                            as_str(): "EN1c",
                                                                        },
                                                                    },
                                                                    is_mutable: false,
                                                                },
                                                            ),
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2028,
                                                            end: 2158,
                                                            as_str(): "match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }",
                                                        },
                                                    },
                                                    AstNode {
                                                        content: ImplicitReturnExpression(
                                                            Expression {
                                                                kind: Match(
                                                                    MatchExpression {
                                                                        value: Expression {
                                                                            kind: Variable(
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "__match_return_var_name_3",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2034,
                                                                                        end: 2038,
                                                                                        as_str(): "EN1c",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            ),
                                                                            span: Span {
                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                ),
                                                                                start: 2034,
                                                                                end: 2038,
                                                                                as_str(): "EN1c",
                                                                            },
                                                                        },
                                                                        branches: [
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2049,
                                                                                                    end: 2052,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 2054,
                                                                                                end: 2057,
                                                                                                as_str(): "Int",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: Variable {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 2058,
                                                                                                end: 2059,
                                                                                                as_str(): "i",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2058,
                                                                                            end: 2059,
                                                                                            as_str(): "i",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2049,
                                                                                        end: 2060,
                                                                                        as_str(): "En1::Int(i)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2064,
                                                                                                            end: 2070,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2064,
                                                                                                    end: 2070,
                                                                                                    as_str(): "assert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            false,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2071,
                                                                                                        end: 2076,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2064,
                                                                                        end: 2077,
                                                                                        as_str(): "assert(false)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2049,
                                                                                    end: 2078,
                                                                                    as_str(): "En1::Int(i) => assert(false),",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2087,
                                                                                                    end: 2090,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 2092,
                                                                                                end: 2095,
                                                                                                as_str(): "Arr",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: CatchAll {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2096,
                                                                                            end: 2097,
                                                                                            as_str(): "_",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2087,
                                                                                        end: 2098,
                                                                                        as_str(): "En1::Arr(_)",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2102,
                                                                                                            end: 2108,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2102,
                                                                                                    end: 2108,
                                                                                                    as_str(): "assert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            false,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2109,
                                                                                                        end: 2114,
                                                                                                        as_str(): "false",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2102,
                                                                                        end: 2115,
                                                                                        as_str(): "assert(false)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2087,
                                                                                    end: 2116,
                                                                                    as_str(): "En1::Arr(_) => assert(false),",
                                                                                },
                                                                            },
                                                                            MatchBranch {
                                                                                scrutinee: EnumScrutinee {
                                                                                    call_path: CallPath {
                                                                                        prefixes: [
                                                                                            BaseIdent {
                                                                                                name_override_opt: None,
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2125,
                                                                                                    end: 2128,
                                                                                                    as_str(): "En1",
                                                                                                },
                                                                                                is_raw_ident: false,
                                                                                            },
                                                                                        ],
                                                                                        suffix: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 2130,
                                                                                                end: 2135,
                                                                                                as_str(): "NoVal",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        is_absolute: false,
                                                                                    },
                                                                                    value: CatchAll {
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                            ),
                                                                                            start: 2125,
                                                                                            end: 2135,
                                                                                            as_str(): "En1::NoVal",
                                                                                        },
                                                                                    },
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2125,
                                                                                        end: 2135,
                                                                                        as_str(): "En1::NoVal",
                                                                                    },
                                                                                },
                                                                                result: Expression {
                                                                                    kind: FunctionApplication(
                                                                                        FunctionApplicationExpression {
                                                                                            call_path_binding: TypeBinding {
                                                                                                inner: CallPath {
                                                                                                    prefixes: [],
                                                                                                    suffix: BaseIdent {
                                                                                                        name_override_opt: None,
                                                                                                        span: Span {
                                                                                                            src (ptr): 0x00007fb1149f7130,
                                                                                                            path: Some(
                                                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                            ),
                                                                                                            start: 2139,
                                                                                                            end: 2145,
                                                                                                            as_str(): "assert",
                                                                                                        },
                                                                                                        is_raw_ident: false,
                                                                                                    },
                                                                                                    is_absolute: false,
                                                                                                },
                                                                                                type_arguments: [],
                                                                                                span: Span {
                                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                                    path: Some(
                                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                    ),
                                                                                                    start: 2139,
                                                                                                    end: 2145,
                                                                                                    as_str(): "assert",
                                                                                                },
                                                                                            },
                                                                                            arguments: [
                                                                                                Expression {
                                                                                                    kind: Literal(
                                                                                                        Boolean(
                                                                                                            true,
                                                                                                        ),
                                                                                                    ),
                                                                                                    span: Span {
                                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                                        path: Some(
                                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                        ),
                                                                                                        start: 2146,
                                                                                                        end: 2150,
                                                                                                        as_str(): "true",
                                                                                                    },
                                                                                                },
                                                                                            ],
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2139,
                                                                                        end: 2151,
                                                                                        as_str(): "assert(true)",
                                                                                    },
                                                                                },
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2125,
                                                                                    end: 2152,
                                                                                    as_str(): "En1::NoVal => assert(true),",
                                                                                },
                                                                            },
                                                                        ],
                                                                    },
                                                                ),
                                                                span: Span {
                                                                    src (ptr): 0x00007fb1149f7130,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                    ),
                                                                    start: 2028,
                                                                    end: 2158,
                                                                    as_str(): "match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }",
                                                                },
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2028,
                                                            end: 2158,
                                                            as_str(): "match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }",
                                                        },
                                                    },
                                                ],
                                                whole_block_span: Span {
                                                    src (ptr): 0x00007fb1149f7130,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                    ),
                                                    start: 2028,
                                                    end: 2158,
                                                    as_str(): "match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }",
                                                },
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 2028,
                                            end: 2158,
                                            as_str(): "match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 2028,
                                    end: 2158,
                                    as_str(): "match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 2201,
                                                                end: 2207,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 2201,
                                                        end: 2207,
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2222,
                                                                                        end: 2224,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2222,
                                                                                        end: 2224,
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2222,
                                                                                    end: 2224,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2222,
                                                                        end: 2224,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Subfield(
                                                                            SubfieldExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 2208,
                                                                                                end: 2215,
                                                                                                as_str(): "ETH_ID0",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2208,
                                                                                        end: 2215,
                                                                                        as_str(): "ETH_ID0",
                                                                                    },
                                                                                },
                                                                                field_to_access: BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2216,
                                                                                        end: 2221,
                                                                                        as_str(): "value",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2208,
                                                                            end: 2221,
                                                                            as_str(): "ETH_ID0.value",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2225,
                                                                                    end: 2238,
                                                                                    as_str(): "ETH_ID0_VALUE",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2225,
                                                                            end: 2238,
                                                                            as_str(): "ETH_ID0_VALUE",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2208,
                                                            end: 2238,
                                                            as_str(): "ETH_ID0.value == ETH_ID0_VALUE",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 2201,
                                            end: 2239,
                                            as_str(): "assert(ETH_ID0.value == ETH_ID0_VALUE)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 2201,
                                    end: 2239,
                                    as_str(): "assert(ETH_ID0.value == ETH_ID0_VALUE)",
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
                                                                src (ptr): 0x00007fb1149f7130,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                ),
                                                                start: 2245,
                                                                end: 2251,
                                                                as_str(): "assert",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        is_absolute: false,
                                                    },
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fb1149f7130,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                        ),
                                                        start: 2245,
                                                        end: 2251,
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
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2262,
                                                                                        end: 2264,
                                                                                        as_str(): "==",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                BaseIdent {
                                                                                    name_override_opt: Some(
                                                                                        "ops",
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2262,
                                                                                        end: 2264,
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
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2262,
                                                                                    end: 2264,
                                                                                    as_str(): "==",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                            is_absolute: true,
                                                                        },
                                                                    },
                                                                    type_arguments: [],
                                                                    span: Span {
                                                                        src (ptr): 0x00007fb1149f7130,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                        ),
                                                                        start: 2262,
                                                                        end: 2264,
                                                                        as_str(): "==",
                                                                    },
                                                                },
                                                                contract_call_params: [],
                                                                arguments: [
                                                                    Expression {
                                                                        kind: Variable(
                                                                            BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2252,
                                                                                    end: 2261,
                                                                                    as_str(): "TUP1_idx2",
                                                                                },
                                                                                is_raw_ident: false,
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2252,
                                                                            end: 2261,
                                                                            as_str(): "TUP1_idx2",
                                                                        },
                                                                    },
                                                                    Expression {
                                                                        kind: TupleIndex(
                                                                            TupleIndexExpression {
                                                                                prefix: Expression {
                                                                                    kind: Variable(
                                                                                        BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fb1149f7130,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                                ),
                                                                                                start: 2265,
                                                                                                end: 2269,
                                                                                                as_str(): "TUP1",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fb1149f7130,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                        ),
                                                                                        start: 2265,
                                                                                        end: 2269,
                                                                                        as_str(): "TUP1",
                                                                                    },
                                                                                },
                                                                                index: 2,
                                                                                index_span: Span {
                                                                                    src (ptr): 0x00007fb1149f7130,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                                    ),
                                                                                    start: 2270,
                                                                                    end: 2271,
                                                                                    as_str(): "2",
                                                                                },
                                                                            },
                                                                        ),
                                                                        span: Span {
                                                                            src (ptr): 0x00007fb1149f7130,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                                            ),
                                                                            start: 2265,
                                                                            end: 2271,
                                                                            as_str(): "TUP1.2",
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fb1149f7130,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                                            ),
                                                            start: 2252,
                                                            end: 2271,
                                                            as_str(): "TUP1_idx2 == TUP1.2",
                                                        },
                                                    },
                                                ],
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 2245,
                                            end: 2272,
                                            as_str(): "assert(TUP1_idx2 == TUP1.2)",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 2245,
                                    end: 2272,
                                    as_str(): "assert(TUP1_idx2 == TUP1.2)",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Literal(
                                            Numeric(
                                                1,
                                            ),
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb1149f7130,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                            ),
                                            start: 2279,
                                            end: 2280,
                                            as_str(): "1",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb1149f7130,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                                    ),
                                    start: 2279,
                                    end: 2280,
                                    as_str(): "1",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb1149f7130,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                            ),
                            start: 920,
                            end: 2282,
                            as_str(): "{\n    const int1 = 1;\n    assert(int1 == INT1 && ZERO_B256 == KEY);\n\n    // initialization through function applications.\n    const eth_id0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);\n    const eth_id1 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001);\n    assert(eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1);\n\n    // tuples and arrays.\n    const t1 = (2, 1, 21);\n    assert(t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2);\n    assert(t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2);\n    const a1 = [1, 2, 3];\n    assert(a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2]);\n    assert(a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2]);\n\n    // enum\n    match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }\n    match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }\n    match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }\n\n    // Struct and enum field access.\n    assert(ETH_ID0.value == ETH_ID0_VALUE);\n    assert(TUP1_idx2 == TUP1.2);\n\n    1\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 903,
                        end: 2282,
                        as_str(): "fn main() -> u64 {\n    const int1 = 1;\n    assert(int1 == INT1 && ZERO_B256 == KEY);\n\n    // initialization through function applications.\n    const eth_id0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);\n    const eth_id1 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001);\n    assert(eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1);\n\n    // tuples and arrays.\n    const t1 = (2, 1, 21);\n    assert(t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2);\n    assert(t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2);\n    const a1 = [1, 2, 3];\n    assert(a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2]);\n    assert(a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2]);\n\n    // enum\n    match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }\n    match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }\n    match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }\n\n    // Struct and enum field access.\n    assert(ETH_ID0.value == ETH_ID0_VALUE);\n    assert(TUP1_idx2 == TUP1.2);\n\n    1\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb1149f7130,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
                        ),
                        start: 916,
                        end: 919,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb1149f7130,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRE3wRpV/const_inits/src/main.sw",
            ),
            start: 903,
            end: 2282,
            as_str(): "fn main() -> u64 {\n    const int1 = 1;\n    assert(int1 == INT1 && ZERO_B256 == KEY);\n\n    // initialization through function applications.\n    const eth_id0 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000000);\n    const eth_id1 = ContractId::from(0x0000000000000000000000000000000000000000000000000000000000000001);\n    assert(eth_id0 == ETH_ID0 && eth_id1 == ETH_ID1);\n\n    // tuples and arrays.\n    const t1 = (2, 1, 21);\n    assert(t1.0 == TUP1.0 && t1.1 == TUP1.1 && t1.2 == TUP1.2);\n    assert(t1.0 == TUP2.0 && t1.1 == TUP2.1 && t1.2 == TUP2.2);\n    const a1 = [1, 2, 3];\n    assert(a1[0] == ARR1[0] && a1[1] == ARR1[1] && a1[2] == ARR1[2]);\n    assert(a1[0] == ARR2[0] && a1[1] == ARR2[1] && a1[2] == ARR2[2]);\n\n    // enum\n    match EN1a {\n        En1::Int(i) => assert(i == 101),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(false),\n    }\n    match EN1b {\n        En1::Int(i) => assert(false),\n        En1::Arr(arr) => {\n            assert(arr[0] == ARR1[0] && arr[1] == ARR1[1] && arr[2] == ARR1[2]);\n        }\n        En1::NoVal => assert(false),\n    }\n    match EN1c {\n        En1::Int(i) => assert(false),\n        En1::Arr(_) => assert(false),\n        En1::NoVal => assert(true),\n    }\n\n    // Struct and enum field access.\n    assert(ETH_ID0.value == ETH_ID0_VALUE);\n    assert(TUP1_idx2 == TUP1.2);\n\n    1\n}",
        },
    },
]
