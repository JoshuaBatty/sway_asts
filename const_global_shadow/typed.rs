TyConstantDeclaration {
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
    value: TyExpression {
        expression: Literal(
            U64(
                1,
            ),
        ),
        return_type: TypeId(
            7251,
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
    return_type: TypeId(
        21,
    ),
    is_configurable: false,
    attributes: {},
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
    span: Span {
        src (ptr): 0x00007fb12be97690,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
        ),
        start: 9,
        end: 35,
        as_str(): "const GLOBAL_VAL: u64 = 1;",
    },
}
TyFunctionDeclaration {
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
    body: TyCodeBlock {
        contents: [
            TyAstNode {
                content: Declaration(
                    ConstantDeclaration(
                        DeclId(
                            545,
                            Span {
                                src (ptr): 0x00007fb12be97690,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                ),
                                start: 66,
                                end: 76,
                                as_str(): "GLOBAL_VAL",
                            },
                        ),
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
            TyAstNode {
                content: Declaration(
                    ConstantDeclaration(
                        DeclId(
                            546,
                            Span {
                                src (ptr): 0x00007fb12be97690,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                ),
                                start: 94,
                                end: 103,
                                as_str(): "LOCAL_VAL",
                            },
                        ),
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
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
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
                            span: Span {
                                src (ptr): 0x00007fb12be97690,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
                                ),
                                start: 122,
                                end: 131,
                                as_str(): "LOCAL_VAL",
                            },
                            mutability: Immutable,
                        },
                        return_type: TypeId(
                            7255,
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
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fb12be97690,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
        ),
        start: 37,
        end: 133,
        as_str(): "fn main() -> u64 {\n    const GLOBAL_VAL = 100;\n    const LOCAL_VAL = GLOBAL_VAL;\n    LOCAL_VAL\n}",
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
        src (ptr): 0x00007fb12be97690,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRQ40Lqr/const_global_shadow/src/main.sw",
        ),
        start: 50,
        end: 53,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

