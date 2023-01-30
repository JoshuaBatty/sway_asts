TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 12,
            end: 16,
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
        src (ptr): 0x00007fe0fc1c14b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
        ),
        start: 9,
        end: 21,
        as_str(): "fn main() {}",
    },
    attributes: {},
    return_type: TypeId(
        7253,
    ),
    initial_return_type: TypeId(
        7252,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0fc1c14b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
        ),
        start: 9,
        end: 18,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 33,
            end: 42,
            as_str(): "Shiftable",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    interface_surface: [
        DeclId(
            545,
            Span {
                src (ptr): 0x00007fe0fc1c14b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                ),
                start: 52,
                end: 55,
                as_str(): "lsh",
            },
        ),
        DeclId(
            547,
            Span {
                src (ptr): 0x00007fe0fc1c14b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                ),
                start: 91,
                end: 94,
                as_str(): "rsh",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Public,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe0fc1c14b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
        ),
        start: 23,
        end: 124,
        as_str(): "pub trait Shiftable {\n    fn lsh(self, other: Self) -> Self;\n    fn rsh(self, other: Self) -> Self;\n}",
    },
}
ImplTrait(
    DeclId(
        552,
        Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 126,
            end: 435,
            as_str(): "impl Shiftable for u64 {\n    fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }\n\n    fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fc1c14b0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
            ),
            start: 440,
            end: 444,
            as_str(): "sqrt",
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
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 520,
                                    end: 521,
                                    as_str(): "z",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        1,
                                    ),
                                ),
                                return_type: TypeId(
                                    7311,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 528,
                                    end: 529,
                                    as_str(): "1",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 522,
                                    end: 525,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 512,
                    end: 530,
                    as_str(): "let mut z:u64 = 1;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 547,
                                    end: 548,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 483,
                                            end: 488,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 555,
                                        end: 560,
                                        as_str(): "value",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    21,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 555,
                                    end: 560,
                                    as_str(): "value",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                21,
                            ),
                            type_ascription: TypeId(
                                21,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 549,
                                    end: 552,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 539,
                    end: 561,
                    as_str(): "let mut y:u64 = value;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 574,
                                    end: 575,
                                    as_str(): "x",
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
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 578,
                                    end: 582,
                                    as_str(): "true",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7312,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 570,
                    end: 583,
                    as_str(): "let x = true;",
                },
            },
            TyAstNode {
                content: Expression(
                    TyExpression {
                        expression: IfExp {
                            condition: TyExpression {
                                expression: VariableExpression {
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fc1c14b0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                            ),
                                            start: 574,
                                            end: 575,
                                            as_str(): "x",
                                        },
                                        is_raw_ident: false,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0fc1c14b0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                        ),
                                        start: 595,
                                        end: 596,
                                        as_str(): "x",
                                    },
                                    mutability: Immutable,
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 595,
                                    end: 596,
                                    as_str(): "x",
                                },
                            },
                            then: TyExpression {
                                expression: CodeBlock(
                                    TyCodeBlock {
                                        contents: [
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Reassignment(
                                                            TyReassignment {
                                                                lhs_base_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 611,
                                                                        end: 612,
                                                                        as_str(): "y",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                lhs_type: TypeId(
                                                                    21,
                                                                ),
                                                                lhs_indices: [],
                                                                rhs: TyExpression {
                                                                    expression: FunctionApplication {
                                                                        call_path: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 617,
                                                                                    end: 620,
                                                                                    as_str(): "rsh",
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
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 304,
                                                                                        end: 308,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 547,
                                                                                                end: 548,
                                                                                                as_str(): "y",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 615,
                                                                                            end: 616,
                                                                                            as_str(): "y",
                                                                                        },
                                                                                        mutability: Mutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 615,
                                                                                        end: 616,
                                                                                        as_str(): "y",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 310,
                                                                                        end: 315,
                                                                                        as_str(): "other",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: Literal(
                                                                                        U64(
                                                                                            32,
                                                                                        ),
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 621,
                                                                                        end: 623,
                                                                                        as_str(): "32",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            553,
                                                                            Span {
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 297,
                                                                                end: 433,
                                                                                as_str(): "fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                            },
                                                                        ),
                                                                        self_state_idx: None,
                                                                        selector: None,
                                                                        type_binding: Some(
                                                                            TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 617,
                                                                                    end: 620,
                                                                                    as_str(): "rsh",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 615,
                                                                        end: 624,
                                                                        as_str(): "y.rsh(32)",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            7321,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 611,
                                                            end: 624,
                                                            as_str(): "y = y.rsh(32)",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 611,
                                                    end: 624,
                                                    as_str(): "y = y.rsh(32)",
                                                },
                                            },
                                            TyAstNode {
                                                content: Expression(
                                                    TyExpression {
                                                        expression: Reassignment(
                                                            TyReassignment {
                                                                lhs_base_name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 638,
                                                                        end: 639,
                                                                        as_str(): "z",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                lhs_type: TypeId(
                                                                    7311,
                                                                ),
                                                                lhs_indices: [],
                                                                rhs: TyExpression {
                                                                    expression: FunctionApplication {
                                                                        call_path: CallPath {
                                                                            prefixes: [],
                                                                            suffix: BaseIdent {
                                                                                name_override_opt: None,
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 644,
                                                                                    end: 647,
                                                                                    as_str(): "lsh",
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
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 162,
                                                                                        end: 166,
                                                                                        as_str(): "self",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: VariableExpression {
                                                                                        name: BaseIdent {
                                                                                            name_override_opt: None,
                                                                                            span: Span {
                                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                                path: Some(
                                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                                ),
                                                                                                start: 520,
                                                                                                end: 521,
                                                                                                as_str(): "z",
                                                                                            },
                                                                                            is_raw_ident: false,
                                                                                        },
                                                                                        span: Span {
                                                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                                                            path: Some(
                                                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                            ),
                                                                                            start: 642,
                                                                                            end: 643,
                                                                                            as_str(): "z",
                                                                                        },
                                                                                        mutability: Mutable,
                                                                                    },
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 642,
                                                                                        end: 643,
                                                                                        as_str(): "z",
                                                                                    },
                                                                                },
                                                                            ),
                                                                            (
                                                                                BaseIdent {
                                                                                    name_override_opt: None,
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 168,
                                                                                        end: 173,
                                                                                        as_str(): "other",
                                                                                    },
                                                                                    is_raw_ident: false,
                                                                                },
                                                                                TyExpression {
                                                                                    expression: Literal(
                                                                                        U64(
                                                                                            16,
                                                                                        ),
                                                                                    ),
                                                                                    return_type: TypeId(
                                                                                        21,
                                                                                    ),
                                                                                    span: Span {
                                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                                        path: Some(
                                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                        ),
                                                                                        start: 648,
                                                                                        end: 650,
                                                                                        as_str(): "16",
                                                                                    },
                                                                                },
                                                                            ),
                                                                        ],
                                                                        function_decl_id: DeclId(
                                                                            554,
                                                                            Span {
                                                                                src (ptr): 0x00007fe0fc1c14b0,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                ),
                                                                                start: 155,
                                                                                end: 291,
                                                                                as_str(): "fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }",
                                                                            },
                                                                        ),
                                                                        self_state_idx: None,
                                                                        selector: None,
                                                                        type_binding: Some(
                                                                            TypeBinding {
                                                                                inner: (),
                                                                                type_arguments: [],
                                                                                span: Span {
                                                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                                                    path: Some(
                                                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                                    ),
                                                                                    start: 644,
                                                                                    end: 647,
                                                                                    as_str(): "lsh",
                                                                                },
                                                                            },
                                                                        ),
                                                                    },
                                                                    return_type: TypeId(
                                                                        21,
                                                                    ),
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fc1c14b0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                                        ),
                                                                        start: 642,
                                                                        end: 651,
                                                                        as_str(): "z.lsh(16)",
                                                                    },
                                                                },
                                                            },
                                                        ),
                                                        return_type: TypeId(
                                                            7328,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fc1c14b0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                            ),
                                                            start: 638,
                                                            end: 651,
                                                            as_str(): "z = z.lsh(16)",
                                                        },
                                                    },
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fc1c14b0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                                    ),
                                                    start: 638,
                                                    end: 651,
                                                    as_str(): "z = z.lsh(16)",
                                                },
                                            },
                                        ],
                                    },
                                ),
                                return_type: TypeId(
                                    7330,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 597,
                                    end: 662,
                                    as_str(): "{\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                                },
                            },
                            else: None,
                        },
                        return_type: TypeId(
                            7333,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 592,
                            end: 662,
                            as_str(): "if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 592,
                    end: 662,
                    as_str(): "if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        }",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fc1c14b0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                    ),
                                    start: 547,
                                    end: 548,
                                    as_str(): "y",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe0fc1c14b0,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                                ),
                                start: 672,
                                end: 673,
                                as_str(): "y",
                            },
                            mutability: Mutable,
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fc1c14b0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                            ),
                            start: 672,
                            end: 673,
                            as_str(): "y",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 672,
                    end: 673,
                    as_str(): "y",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 445,
                    end: 449,
                    as_str(): "gas_",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
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
                src (ptr): 0x00007fe0fc1c14b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                ),
                start: 451,
                end: 454,
                as_str(): "u64",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 456,
                    end: 463,
                    as_str(): "amount_",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
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
                src (ptr): 0x00007fe0fc1c14b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                ),
                start: 465,
                end: 468,
                as_str(): "u64",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 470,
                    end: 475,
                    as_str(): "coin_",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
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
                src (ptr): 0x00007fe0fc1c14b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                ),
                start: 477,
                end: 481,
                as_str(): "b256",
            },
        },
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fc1c14b0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                    ),
                    start: 483,
                    end: 488,
                    as_str(): "value",
                },
                is_raw_ident: false,
            },
            is_reference: false,
            is_mutable: false,
            mutability_span: Span {
                src (ptr): 0x00007fe0fc01dd50,
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
                src (ptr): 0x00007fe0fc1c14b0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
                ),
                start: 490,
                end: 493,
                as_str(): "u64",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fc1c14b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
        ),
        start: 437,
        end: 675,
        as_str(): "fn sqrt(gas_: u64, amount_: u64, coin_: b256, value: u64)-> u64  {\n        let mut z:u64 = 1;\n        let mut y:u64 = value;\n        let x = true;\n        if x {\n            y = y.rsh(32);\n            z = z.lsh(16);\n        };\n        y\n}",
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
        src (ptr): 0x00007fe0fc1c14b0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIR7o6lSv/if_implicit_unit/src/main.sw",
        ),
        start: 497,
        end: 500,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

