[
    AstNode {
        content: Declaration(
            ConstantDeclaration(
                ConstantDeclaration {
                    name: BaseIdent {
                        name_override_opt: None,
                        span: Span {
                            src (ptr): 0x00007fb12be97690,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                            ),
                            start: 15,
                            end: 25,
                            as_str(): "GLOBAL_VAL",
                        },
                        is_raw_ident: false,
                    },
                    attributes: {},
                    type_ascription: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_ascription_span: Some(
                        Span {
                            src (ptr): 0x00007fb12be97690,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                            ),
                            start: 27,
                            end: 30,
                            as_str(): "u64",
                        },
                    ),
                    value: Expression {
                        kind: Literal(
                            Numeric(
                                1,
                            ),
                        ),
                        span: Span {
                            src (ptr): 0x00007fb12be97690,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                            ),
                            start: 33,
                            end: 34,
                            as_str(): "1",
                        },
                    },
                    visibility: Private,
                    is_configurable: false,
                    span: Span {
                        src (ptr): 0x00007fb12be97690,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                        ),
                        start: 9,
                        end: 35,
                        as_str(): "const GLOBAL_VAL: u64 = 1;",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12be97690,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
            ),
            start: 9,
            end: 35,
            as_str(): "const GLOBAL_VAL: u64 = 1;",
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
                            src (ptr): 0x00007fb12be97690,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                            ),
                            start: 40,
                            end: 44,
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
                                                    src (ptr): 0x00007fb12be97690,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                    ),
                                                    start: 66,
                                                    end: 76,
                                                    as_str(): "GLOBAL_VAL",
                                                },
                                                is_raw_ident: false,
                                            },
                                            attributes: {},
                                            type_ascription: Unknown,
                                            type_ascription_span: None,
                                            value: Expression {
                                                kind: Literal(
                                                    Numeric(
                                                        100,
                                                    ),
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12be97690,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                    ),
                                                    start: 79,
                                                    end: 82,
                                                    as_str(): "100",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb12be97690,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                ),
                                                start: 60,
                                                end: 83,
                                                as_str(): "const GLOBAL_VAL = 100;",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12be97690,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                    ),
                                    start: 60,
                                    end: 83,
                                    as_str(): "const GLOBAL_VAL = 100;",
                                },
                            },
                            AstNode {
                                content: Declaration(
                                    ConstantDeclaration(
                                        ConstantDeclaration {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12be97690,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                    ),
                                                    start: 94,
                                                    end: 103,
                                                    as_str(): "LOCAL_VAL",
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
                                                            src (ptr): 0x00007fb12be97690,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                            ),
                                                            start: 106,
                                                            end: 116,
                                                            as_str(): "GLOBAL_VAL",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fb12be97690,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                    ),
                                                    start: 106,
                                                    end: 116,
                                                    as_str(): "GLOBAL_VAL",
                                                },
                                            },
                                            visibility: Private,
                                            is_configurable: false,
                                            span: Span {
                                                src (ptr): 0x00007fb12be97690,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                ),
                                                start: 88,
                                                end: 117,
                                                as_str(): "const LOCAL_VAL = GLOBAL_VAL;",
                                            },
                                        },
                                    ),
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12be97690,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                    ),
                                    start: 88,
                                    end: 117,
                                    as_str(): "const LOCAL_VAL = GLOBAL_VAL;",
                                },
                            },
                            AstNode {
                                content: ImplicitReturnExpression(
                                    Expression {
                                        kind: Variable(
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fb12be97690,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                                    ),
                                                    start: 122,
                                                    end: 131,
                                                    as_str(): "LOCAL_VAL",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fb12be97690,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                            ),
                                            start: 122,
                                            end: 131,
                                            as_str(): "LOCAL_VAL",
                                        },
                                    },
                                ),
                                span: Span {
                                    src (ptr): 0x00007fb12be97690,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                    ),
                                    start: 122,
                                    end: 131,
                                    as_str(): "LOCAL_VAL",
                                },
                            },
                        ],
                        whole_block_span: Span {
                            src (ptr): 0x00007fb12be97690,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                            ),
                            start: 54,
                            end: 133,
                            as_str(): "{\n    const GLOBAL_VAL = 100;\n    const LOCAL_VAL = GLOBAL_VAL;\n    LOCAL_VAL\n}",
                        },
                    },
                    parameters: [],
                    span: Span {
                        src (ptr): 0x00007fb12be97690,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                        ),
                        start: 37,
                        end: 133,
                        as_str(): "fn main() -> u64 {\n    const GLOBAL_VAL = 100;\n    const LOCAL_VAL = GLOBAL_VAL;\n    LOCAL_VAL\n}",
                    },
                    return_type: UnsignedInteger(
                        SixtyFour,
                    ),
                    type_parameters: [],
                    return_type_span: Span {
                        src (ptr): 0x00007fb12be97690,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                        ),
                        start: 50,
                        end: 53,
                        as_str(): "u64",
                    },
                },
            ),
        ),
        span: Span {
            src (ptr): 0x00007fb12be97690,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
            ),
            start: 37,
            end: 133,
            as_str(): "fn main() -> u64 {\n    const GLOBAL_VAL = 100;\n    const LOCAL_VAL = GLOBAL_VAL;\n    LOCAL_VAL\n}",
        },
    },
]
