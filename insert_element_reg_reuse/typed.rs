ImplTrait(
    DeclId(
        13333,
        Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 9,
            end: 258,
            as_str(): "impl core::ops::Eq for [u64; 2] {\n    fn eq(self, other: Self) -> bool {\n        let mut i = 0;\n        while i < 2 {\n            if self[i] != other[i] {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        13809,
        Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 260,
            end: 624,
            as_str(): "impl core::ops::Eq for Vec<[u64; 2]> {\n    fn eq(self, other: Self) -> bool {\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 629,
            end: 636,
            as_str(): "tester1",
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
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 671,
                                    end: 679,
                                    as_str(): "expected",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 682,
                                                    end: 685,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 687,
                                                end: 690,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        14458,
                                        Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 682,
                                                end: 690,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    32685,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 682,
                                    end: 692,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                32685,
                            ),
                            type_ascription: TypeId(
                                32561,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 663,
                    end: 693,
                    as_str(): "let mut expected = Vec::new();",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 707,
                                        end: 711,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 671,
                                                    end: 679,
                                                    as_str(): "expected",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 698,
                                                end: 706,
                                                as_str(): "expected",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            32685,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 698,
                                            end: 706,
                                            as_str(): "expected",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 713,
                                                        end: 714,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 716,
                                                        end: 717,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            33540,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 712,
                                            end: 718,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                14957,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 707,
                                        end: 711,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            33541,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 698,
                            end: 719,
                            as_str(): "expected.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 698,
                    end: 719,
                    as_str(): "expected.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 734,
                                        end: 738,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 671,
                                                    end: 679,
                                                    as_str(): "expected",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 725,
                                                end: 733,
                                                as_str(): "expected",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            32685,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 725,
                                            end: 733,
                                            as_str(): "expected",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 740,
                                                        end: 741,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 743,
                                                        end: 744,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            33550,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 739,
                                            end: 745,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                14958,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 734,
                                        end: 738,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            33551,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 725,
                            end: 746,
                            as_str(): "expected.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 725,
                    end: 746,
                    as_str(): "expected.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 753,
                                        end: 759,
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
                                            src (ptr): 0x00007fe0d3dfa310,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 764,
                                                            end: 766,
                                                            as_str(): "==",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 764,
                                                            end: 766,
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 764,
                                                        end: 766,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 309,
                                                            end: 313,
                                                            as_str(): "self",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 637,
                                                                    end: 640,
                                                                    as_str(): "arg",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 760,
                                                                end: 763,
                                                                as_str(): "arg",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            32235,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 760,
                                                            end: 763,
                                                            as_str(): "arg",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 315,
                                                            end: 320,
                                                            as_str(): "other",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    TyExpression {
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 671,
                                                                    end: 679,
                                                                    as_str(): "expected",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 767,
                                                                end: 775,
                                                                as_str(): "expected",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            32685,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 767,
                                                            end: 775,
                                                            as_str(): "expected",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                14960,
                                                Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 303,
                                                    end: 622,
                                                    as_str(): "fn eq(self, other: Self) -> bool {\n        if self.len() != other.len() {\n            return false;\n        }\n        let mut i = 0;\n        while i < self.len() {\n            if self.get(i).unwrap() != other.get(i).unwrap() {\n                return false;\n            }\n            i += 1;\n        }\n        true\n    }",
                                                },
                                            ),
                                            self_state_idx: None,
                                            selector: None,
                                            type_binding: Some(
                                                TypeBinding {
                                                    inner: (),
                                                    type_arguments: [],
                                                    span: Span {
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 764,
                                                        end: 766,
                                                        as_str(): "==",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 760,
                                            end: 775,
                                            as_str(): "arg == expected",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                14961,
                                Span {
                                    src (ptr): 0x00007fe0d3dfa310,
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 753,
                                        end: 759,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            33556,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 753,
                            end: 776,
                            as_str(): "assert(arg == expected)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 753,
                    end: 776,
                    as_str(): "assert(arg == expected)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 637,
                    end: 640,
                    as_str(): "arg",
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
                32235,
            ),
            initial_type_id: TypeId(
                32232,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0c6ce4c30,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                ),
                start: 642,
                end: 655,
                as_str(): "Vec<[u64; 2]>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c6ce4c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
        ),
        start: 626,
        end: 779,
        as_str(): "fn tester1(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg == expected);\n}",
    },
    attributes: {},
    return_type: TypeId(
        32560,
    ),
    initial_return_type: TypeId(
        32559,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0c6ce4c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
        ),
        start: 626,
        end: 656,
        as_str(): "fn tester1(arg: Vec<[u64; 2]>)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 784,
            end: 791,
            as_str(): "tester2",
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
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 826,
                                    end: 834,
                                    as_str(): "expected",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 837,
                                                    end: 840,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 842,
                                                end: 845,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        15611,
                                        Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 837,
                                                end: 845,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    34012,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 837,
                                    end: 847,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                34012,
                            ),
                            type_ascription: TypeId(
                                33888,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 818,
                    end: 848,
                    as_str(): "let mut expected = Vec::new();",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 862,
                                        end: 866,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 826,
                                                    end: 834,
                                                    as_str(): "expected",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 853,
                                                end: 861,
                                                as_str(): "expected",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            34012,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 853,
                                            end: 861,
                                            as_str(): "expected",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 868,
                                                        end: 869,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 871,
                                                        end: 872,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            34867,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 867,
                                            end: 873,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16110,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 862,
                                        end: 866,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            34868,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 853,
                            end: 874,
                            as_str(): "expected.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 853,
                    end: 874,
                    as_str(): "expected.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 889,
                                        end: 893,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 826,
                                                    end: 834,
                                                    as_str(): "expected",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 880,
                                                end: 888,
                                                as_str(): "expected",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            34012,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 880,
                                            end: 888,
                                            as_str(): "expected",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 895,
                                                        end: 896,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 898,
                                                        end: 899,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            34877,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 894,
                                            end: 900,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16111,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 889,
                                        end: 893,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            34878,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 880,
                            end: 901,
                            as_str(): "expected.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 880,
                    end: 901,
                    as_str(): "expected.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 908,
                                        end: 914,
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
                                            src (ptr): 0x00007fe0d3dfa310,
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
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 919,
                                                            end: 921,
                                                            as_str(): "!=",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    BaseIdent {
                                                        name_override_opt: Some(
                                                            "ops",
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 919,
                                                            end: 921,
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 919,
                                                        end: 921,
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
                                                            src (ptr): 0x00007fe0d3ba81f0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 792,
                                                                    end: 795,
                                                                    as_str(): "arg",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 915,
                                                                end: 918,
                                                                as_str(): "arg",
                                                            },
                                                            mutability: Immutable,
                                                        },
                                                        return_type: TypeId(
                                                            33562,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 915,
                                                            end: 918,
                                                            as_str(): "arg",
                                                        },
                                                    },
                                                ),
                                                (
                                                    BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0d3ba81f0,
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
                                                        expression: VariableExpression {
                                                            name: BaseIdent {
                                                                name_override_opt: None,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                    ),
                                                                    start: 826,
                                                                    end: 834,
                                                                    as_str(): "expected",
                                                                },
                                                                is_raw_ident: false,
                                                            },
                                                            span: Span {
                                                                src (ptr): 0x00007fe0c6ce4c30,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                                ),
                                                                start: 922,
                                                                end: 930,
                                                                as_str(): "expected",
                                                            },
                                                            mutability: Mutable,
                                                        },
                                                        return_type: TypeId(
                                                            34012,
                                                        ),
                                                        span: Span {
                                                            src (ptr): 0x00007fe0c6ce4c30,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                            ),
                                                            start: 922,
                                                            end: 930,
                                                            as_str(): "expected",
                                                        },
                                                    },
                                                ),
                                            ],
                                            function_decl_id: DeclId(
                                                16113,
                                                Span {
                                                    src (ptr): 0x00007fe0d3ba81f0,
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 919,
                                                        end: 921,
                                                        as_str(): "!=",
                                                    },
                                                },
                                            ),
                                        },
                                        return_type: TypeId(
                                            71,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 915,
                                            end: 930,
                                            as_str(): "arg != expected",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16114,
                                Span {
                                    src (ptr): 0x00007fe0d3dfa310,
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 908,
                                        end: 914,
                                        as_str(): "assert",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            34883,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 908,
                            end: 931,
                            as_str(): "assert(arg != expected)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 908,
                    end: 931,
                    as_str(): "assert(arg != expected)",
                },
            },
        ],
    },
    parameters: [
        TyFunctionParameter {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 792,
                    end: 795,
                    as_str(): "arg",
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
                33562,
            ),
            initial_type_id: TypeId(
                33559,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0c6ce4c30,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                ),
                start: 797,
                end: 810,
                as_str(): "Vec<[u64; 2]>",
            },
        },
    ],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c6ce4c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
        ),
        start: 781,
        end: 934,
        as_str(): "fn tester2(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg != expected);\n}",
    },
    attributes: {},
    return_type: TypeId(
        33887,
    ),
    initial_return_type: TypeId(
        33886,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0c6ce4c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
        ),
        start: 781,
        end: 811,
        as_str(): "fn tester2(arg: Vec<[u64; 2]>)",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0c6ce4c30,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
            ),
            start: 939,
            end: 943,
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
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 966,
                                    end: 970,
                                    as_str(): "arg1",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 973,
                                                    end: 976,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 978,
                                                end: 981,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        16365,
                                        Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 973,
                                                end: 981,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    35010,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 973,
                                    end: 983,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                35010,
                            ),
                            type_ascription: TypeId(
                                34886,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 958,
                    end: 984,
                    as_str(): "let mut arg1 = Vec::new();",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 992,
                                        end: 996,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 966,
                                                    end: 970,
                                                    as_str(): "arg1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 987,
                                                end: 991,
                                                as_str(): "arg1",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            35010,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 987,
                                            end: 991,
                                            as_str(): "arg1",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 998,
                                                        end: 999,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1001,
                                                        end: 1002,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            35865,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 997,
                                            end: 1003,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16864,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 992,
                                        end: 996,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            35866,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 987,
                            end: 1004,
                            as_str(): "arg1.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 987,
                    end: 1004,
                    as_str(): "arg1.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1013,
                                        end: 1017,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 966,
                                                    end: 970,
                                                    as_str(): "arg1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1008,
                                                end: 1012,
                                                as_str(): "arg1",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            35010,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1008,
                                            end: 1012,
                                            as_str(): "arg1",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1019,
                                                        end: 1020,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1022,
                                                        end: 1023,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            35875,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1018,
                                            end: 1024,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16865,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1013,
                                        end: 1017,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            35876,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1008,
                            end: 1025,
                            as_str(): "arg1.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1008,
                    end: 1025,
                    as_str(): "arg1.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1029,
                                        end: 1036,
                                        as_str(): "tester1",
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
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 637,
                                            end: 640,
                                            as_str(): "arg",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 966,
                                                    end: 970,
                                                    as_str(): "arg1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1037,
                                                end: 1041,
                                                as_str(): "arg1",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            35010,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1037,
                                            end: 1041,
                                            as_str(): "arg1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                16867,
                                Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 626,
                                    end: 779,
                                    as_str(): "fn tester1(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg == expected);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1029,
                                        end: 1036,
                                        as_str(): "tester1",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            35879,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1029,
                            end: 1042,
                            as_str(): "tester1(arg1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1029,
                    end: 1042,
                    as_str(): "tester1(arg1)",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1055,
                                    end: 1059,
                                    as_str(): "arg2",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 1062,
                                                    end: 1065,
                                                    as_str(): "Vec",
                                                },
                                                is_raw_ident: false,
                                            },
                                        ],
                                        suffix: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1067,
                                                end: 1070,
                                                as_str(): "new",
                                            },
                                            is_raw_ident: false,
                                        },
                                        is_absolute: false,
                                    },
                                    contract_call_params: {},
                                    arguments: [],
                                    function_decl_id: DeclId(
                                        17117,
                                        Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 1761,
                                            end: 1866,
                                            as_str(): "pub fn new() -> Self {\n        Self {\n            buf: RawVec::new(),\n            len: 0,\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1062,
                                                end: 1070,
                                                as_str(): "Vec::new",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    36004,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 1062,
                                    end: 1072,
                                    as_str(): "Vec::new()",
                                },
                            },
                            mutability: Mutable,
                            return_type: TypeId(
                                36004,
                            ),
                            type_ascription: TypeId(
                                35880,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1047,
                    end: 1073,
                    as_str(): "let mut arg2 = Vec::new();",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1081,
                                        end: 1085,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 1055,
                                                    end: 1059,
                                                    as_str(): "arg2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1076,
                                                end: 1080,
                                                as_str(): "arg2",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            36004,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1076,
                                            end: 1080,
                                            as_str(): "arg2",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1087,
                                                        end: 1088,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1090,
                                                        end: 1091,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            36859,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1086,
                                            end: 1092,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                17616,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1081,
                                        end: 1085,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36860,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1076,
                            end: 1093,
                            as_str(): "arg2.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1076,
                    end: 1093,
                    as_str(): "arg2.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1102,
                                        end: 1106,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 1055,
                                                    end: 1059,
                                                    as_str(): "arg2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1097,
                                                end: 1101,
                                                as_str(): "arg2",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            36004,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1097,
                                            end: 1101,
                                            as_str(): "arg2",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1108,
                                                        end: 1109,
                                                        as_str(): "0",
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1111,
                                                        end: 1112,
                                                        as_str(): "2",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            36869,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1107,
                                            end: 1113,
                                            as_str(): "[0, 2]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                17617,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1102,
                                        end: 1106,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36870,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1097,
                            end: 1114,
                            as_str(): "arg2.push([0, 2])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1097,
                    end: 1114,
                    as_str(): "arg2.push([0, 2])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1118,
                                        end: 1125,
                                        as_str(): "tester2",
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
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 792,
                                            end: 795,
                                            as_str(): "arg",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 1055,
                                                    end: 1059,
                                                    as_str(): "arg2",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1126,
                                                end: 1130,
                                                as_str(): "arg2",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            36004,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1126,
                                            end: 1130,
                                            as_str(): "arg2",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                17619,
                                Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 781,
                                    end: 934,
                                    as_str(): "fn tester2(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg != expected);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1118,
                                        end: 1125,
                                        as_str(): "tester2",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36873,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1118,
                            end: 1131,
                            as_str(): "tester2(arg2)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1118,
                    end: 1131,
                    as_str(): "tester2(arg2)",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1141,
                                        end: 1145,
                                        as_str(): "push",
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
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2954,
                                            end: 2958,
                                            as_str(): "self",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 966,
                                                    end: 970,
                                                    as_str(): "arg1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1136,
                                                end: 1140,
                                                as_str(): "arg1",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            35010,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1136,
                                            end: 1140,
                                            as_str(): "arg1",
                                        },
                                    },
                                ),
                                (
                                    BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0d36797f0,
                                            path: Some(
                                                "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                            ),
                                            start: 2960,
                                            end: 2965,
                                            as_str(): "value",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: Array {
                                            contents: [
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1147,
                                                        end: 1148,
                                                        as_str(): "0",
                                                    },
                                                },
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
                                                        src (ptr): 0x00007fe0c6ce4c30,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                        ),
                                                        start: 1150,
                                                        end: 1151,
                                                        as_str(): "1",
                                                    },
                                                },
                                            ],
                                        },
                                        return_type: TypeId(
                                            36882,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1146,
                                            end: 1152,
                                            as_str(): "[0, 1]",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                17620,
                                Span {
                                    src (ptr): 0x00007fe0d36797f0,
                                    path: Some(
                                        "/home/josh/Documents/rust/fuel/sway/sway-lib-std/src/vec.sw",
                                    ),
                                    start: 2934,
                                    end: 3408,
                                    as_str(): "pub fn push(ref mut self, value: T) {\n        // If there is insufficient capacity, grow the buffer.\n        if self.len == self.buf.capacity() {\n            self.buf.grow();\n        };\n\n        // Get a pointer to the end of the buffer, where the new element will\n        // be inserted.\n        let end = self.buf.ptr().add::<T>(self.len);\n\n        // Write `value` at pointer `end`\n        end.write::<T>(value);\n\n        // Increment length.\n        self.len += 1;\n    }",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1141,
                                        end: 1145,
                                        as_str(): "push",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36883,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1136,
                            end: 1153,
                            as_str(): "arg1.push([0, 1])",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1136,
                    end: 1153,
                    as_str(): "arg1.push([0, 1])",
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
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1157,
                                        end: 1164,
                                        as_str(): "tester2",
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
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 792,
                                            end: 795,
                                            as_str(): "arg",
                                        },
                                        is_raw_ident: false,
                                    },
                                    TyExpression {
                                        expression: VariableExpression {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0c6ce4c30,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                    ),
                                                    start: 966,
                                                    end: 970,
                                                    as_str(): "arg1",
                                                },
                                                is_raw_ident: false,
                                            },
                                            span: Span {
                                                src (ptr): 0x00007fe0c6ce4c30,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                                ),
                                                start: 1165,
                                                end: 1169,
                                                as_str(): "arg1",
                                            },
                                            mutability: Mutable,
                                        },
                                        return_type: TypeId(
                                            35010,
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0c6ce4c30,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                            ),
                                            start: 1165,
                                            end: 1169,
                                            as_str(): "arg1",
                                        },
                                    },
                                ),
                            ],
                            function_decl_id: DeclId(
                                17622,
                                Span {
                                    src (ptr): 0x00007fe0c6ce4c30,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                    ),
                                    start: 781,
                                    end: 934,
                                    as_str(): "fn tester2(arg: Vec<[u64; 2]>) {\n    let mut expected = Vec::new();\n    expected.push([0, 1]);\n    expected.push([0, 1]);\n\n    assert(arg != expected);\n}",
                                },
                            ),
                            self_state_idx: None,
                            selector: None,
                            type_binding: Some(
                                TypeBinding {
                                    inner: (),
                                    type_arguments: [],
                                    span: Span {
                                        src (ptr): 0x00007fe0c6ce4c30,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                                        ),
                                        start: 1157,
                                        end: 1164,
                                        as_str(): "tester2",
                                    },
                                },
                            ),
                        },
                        return_type: TypeId(
                            36886,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1157,
                            end: 1170,
                            as_str(): "tester2(arg1)",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1157,
                    end: 1170,
                    as_str(): "tester2(arg1)",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U64(
                                1,
                            ),
                        ),
                        return_type: TypeId(
                            36887,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0c6ce4c30,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                            ),
                            start: 1175,
                            end: 1176,
                            as_str(): "1",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0c6ce4c30,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
                    ),
                    start: 1175,
                    end: 1176,
                    as_str(): "1",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0c6ce4c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
        ),
        start: 936,
        end: 1178,
        as_str(): "fn main() -> u64 {\n\n  let mut arg1 = Vec::new();\n  arg1.push([0, 1]);\n  arg1.push([0, 1]);\n  tester1(arg1);\n\n  let mut arg2 = Vec::new();\n  arg2.push([0, 1]);\n  arg2.push([0, 2]);\n  tester2(arg2);\n\n  arg1.push([0, 1]);\n  tester2(arg1);\n\n  1\n}",
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
        src (ptr): 0x00007fe0c6ce4c30,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRbAyodQ/insert_element_reg_reuse/src/main.sw",
        ),
        start: 949,
        end: 952,
        as_str(): "u64",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

