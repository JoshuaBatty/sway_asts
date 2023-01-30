TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 142,
            end: 151,
            as_str(): "Shiftable",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    interface_surface: [
        DeclId(
            544,
            Span {
                src (ptr): 0x00007fe043ecf990,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                ),
                start: 161,
                end: 164,
                as_str(): "lsh",
            },
        ),
        DeclId(
            546,
            Span {
                src (ptr): 0x00007fe043ecf990,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                ),
                start: 199,
                end: 202,
                as_str(): "rsh",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Public,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe043ecf990,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
        ),
        start: 132,
        end: 232,
        as_str(): "pub trait Shiftable {\n    fn lsh(self, other: u64) -> Self;\n    fn rsh(self, other: u64) -> Self;\n\n}",
    },
}
ImplTrait(
    DeclId(
        551,
        Span {
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 234,
            end: 543,
            as_str(): "impl Shiftable for u64 {\n    fn lsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            sll r3 r1 r2;\n            r3: u64\n        }\n    }\n\n    fn rsh(self, other: u64) -> Self {\n        asm(r1 : self, r2: other, r3) {\n            srl r3 r1 r2;\n            r3: u64\n        }\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 548,
            end: 551,
            as_str(): "foo",
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
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 575,
                                    end: 576,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: Literal(
                                    U64(
                                        4,
                                    ),
                                ),
                                return_type: TypeId(
                                    7305,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 584,
                                    end: 585,
                                    as_str(): "4",
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
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 578,
                                    end: 581,
                                    as_str(): "u64",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe043ecf990,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                    ),
                    start: 567,
                    end: 586,
                    as_str(): "let mut x: u64 = 4;",
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
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 591,
                                        end: 592,
                                        as_str(): "x",
                                    },
                                    is_raw_ident: false,
                                },
                                lhs_type: TypeId(
                                    7305,
                                ),
                                lhs_indices: [],
                                rhs: TyExpression {
                                    expression: FunctionApplication {
                                        call_path: CallPath {
                                            prefixes: [
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "core",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 597,
                                                        end: 598,
                                                        as_str(): "+",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                BaseIdent {
                                                    name_override_opt: Some(
                                                        "ops",
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 597,
                                                        end: 598,
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
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 597,
                                                    end: 598,
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
                                                        src (ptr): 0x00007fe043e90950,
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
                                                    expression: Literal(
                                                        U64(
                                                            5,
                                                        ),
                                                    ),
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 595,
                                                        end: 596,
                                                        as_str(): "5",
                                                    },
                                                },
                                            ),
                                            (
                                                BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe043e90950,
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
                                                    expression: Literal(
                                                        U64(
                                                            2,
                                                        ),
                                                    ),
                                                    return_type: TypeId(
                                                        21,
                                                    ),
                                                    span: Span {
                                                        src (ptr): 0x00007fe043ecf990,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                        ),
                                                        start: 599,
                                                        end: 600,
                                                        as_str(): "2",
                                                    },
                                                },
                                            ),
                                        ],
                                        function_decl_id: DeclId(
                                            552,
                                            Span {
                                                src (ptr): 0x00007fe043e90950,
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
                                                    src (ptr): 0x00007fe043ecf990,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                                    ),
                                                    start: 597,
                                                    end: 598,
                                                    as_str(): "+",
                                                },
                                            },
                                        ),
                                    },
                                    return_type: TypeId(
                                        21,
                                    ),
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 595,
                                        end: 600,
                                        as_str(): "5 + 2",
                                    },
                                },
                            },
                        ),
                        return_type: TypeId(
                            7313,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 591,
                            end: 600,
                            as_str(): "x = 5 + 2",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe043ecf990,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                    ),
                    start: 591,
                    end: 600,
                    as_str(): "x = 5 + 2",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: VariableExpression {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 575,
                                    end: 576,
                                    as_str(): "x",
                                },
                                is_raw_ident: false,
                            },
                            span: Span {
                                src (ptr): 0x00007fe043ecf990,
                                path: Some(
                                    "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                ),
                                start: 606,
                                end: 607,
                                as_str(): "x",
                            },
                            mutability: Mutable,
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 606,
                            end: 607,
                            as_str(): "x",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe043ecf990,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                    ),
                    start: 606,
                    end: 607,
                    as_str(): "x",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe043ecf990,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
        ),
        start: 545,
        end: 609,
        as_str(): "fn foo() -> u64 {\n    let mut x: u64 = 4;\n    x = 5 + 2;\n    x\n}",
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
        src (ptr): 0x00007fe043ecf990,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
        ),
        start: 557,
        end: 560,
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
            src (ptr): 0x00007fe043ecf990,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
            ),
            start: 614,
            end: 618,
            as_str(): "main",
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
                                prefixes: [],
                                suffix: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 632,
                                        end: 635,
                                        as_str(): "foo",
                                    },
                                    is_raw_ident: false,
                                },
                                is_absolute: false,
                            },
                            contract_call_params: {},
                            arguments: [],
                            function_decl_id: DeclId(
                                555,
                                Span {
                                    src (ptr): 0x00007fe043ecf990,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                    ),
                                    start: 545,
                                    end: 609,
                                    as_str(): "fn foo() -> u64 {\n    let mut x: u64 = 4;\n    x = 5 + 2;\n    x\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe043ecf990,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                                        ),
                                        start: 632,
                                        end: 635,
                                        as_str(): "foo",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            21,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe043ecf990,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                            ),
                            start: 632,
                            end: 637,
                            as_str(): "foo()",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe043ecf990,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
                    ),
                    start: 632,
                    end: 637,
                    as_str(): "foo()",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe043ecf990,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
        ),
        start: 611,
        end: 639,
        as_str(): "fn main() -> u64 {\n  foo()\n}",
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
        src (ptr): 0x00007fe043ecf990,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRZFj75R/trait_override_bug/src/main.sw",
        ),
        start: 624,
        end: 627,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

