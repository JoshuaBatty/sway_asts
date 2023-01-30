

TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 49,
            end: 52,
            as_str(): "Rgb",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 59,
                    end: 62,
                    as_str(): "red",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 59,
                end: 67,
                as_str(): "red: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 64,
                end: 67,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 73,
                    end: 78,
                    as_str(): "green",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 73,
                end: 83,
                as_str(): "green: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 80,
                end: 83,
                as_str(): "u64",
            },
            attributes: {},
        },
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 89,
                    end: 93,
                    as_str(): "blue",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                21,
            ),
            initial_type_id: TypeId(
                21,
            ),
            span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 89,
                end: 98,
                as_str(): "blue: u64",
            },
            type_span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 95,
                end: 98,
                as_str(): "u64",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe0fb117fe0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
        ),
        start: 42,
        end: 101,
        as_str(): "struct Rgb {\n    red: u64,\n    green: u64,\n    blue: u64,\n}",
    },
    attributes: {},
}
TyTraitDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 109,
            end: 114,
            as_str(): "Color",
        },
        is_raw_ident: false,
    },
    type_parameters: [],
    interface_surface: [
        DeclId(
            545,
            Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 124,
                end: 127,
                as_str(): "rgb",
            },
        ),
    ],
    methods: [],
    supertraits: [],
    visibility: Private,
    attributes: {},
    span: Span {
        src (ptr): 0x00007fe0fb117fe0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
        ),
        start: 103,
        end: 143,
        as_str(): "trait Color {\n    fn rgb(self) -> Rgb;\n}",
    },
}
TyEnumDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 150,
            end: 162,
            as_str(): "PrimaryColor",
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
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 169,
                    end: 172,
                    as_str(): "Red",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7268,
            ),
            initial_type_id: TypeId(
                7267,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 174,
                end: 176,
                as_str(): "()",
            },
            tag: 0,
            span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 169,
                end: 176,
                as_str(): "Red: ()",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 182,
                    end: 187,
                    as_str(): "Green",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7270,
            ),
            initial_type_id: TypeId(
                7269,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 189,
                end: 191,
                as_str(): "()",
            },
            tag: 1,
            span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 182,
                end: 191,
                as_str(): "Green: ()",
            },
            attributes: {},
        },
        TyEnumVariant {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 197,
                    end: 201,
                    as_str(): "Blue",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                7272,
            ),
            initial_type_id: TypeId(
                7271,
            ),
            type_span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 203,
                end: 205,
                as_str(): "()",
            },
            tag: 2,
            span: Span {
                src (ptr): 0x00007fe0fb117fe0,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                ),
                start: 197,
                end: 205,
                as_str(): "Blue: ()",
            },
            attributes: {},
        },
    ],
    span: Span {
        src (ptr): 0x00007fe0fb117fe0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
        ),
        start: 145,
        end: 208,
        as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
    },
    visibility: Private,
}
ImplTrait(
    DeclId(
        555,
        Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 210,
            end: 389,
            as_str(): "impl core::ops::Eq for PrimaryColor {\n    fn eq(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        558,
        Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 391,
            end: 711,
            as_str(): "impl core::ops::Ord for PrimaryColor {\n    fn lt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            lt r3 r1 r2;\n            r3: bool\n        }\n    }\n    fn gt(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            gt r3 r1 r2;\n            r3: bool\n        }\n    }\n}",
        },
    ),
)
ImplTrait(
    DeclId(
        566,
        Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 713,
            end: 1528,
            as_str(): "impl Color for PrimaryColor {\n    // TODO: when we support match statements, change this to a match statement\n    fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }\n}",
        },
    ),
)
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe0fb117fe0,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
            ),
            start: 1533,
            end: 1537,
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
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1557,
                                    end: 1568,
                                    as_str(): "first_color",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 150,
                                                end: 162,
                                                as_str(): "PrimaryColor",
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 172,
                                                        as_str(): "Red",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7268,
                                                ),
                                                initial_type_id: TypeId(
                                                    7267,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 176,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 176,
                                                    as_str(): "Red: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 182,
                                                        end: 187,
                                                        as_str(): "Green",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7270,
                                                ),
                                                initial_type_id: TypeId(
                                                    7269,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 189,
                                                    end: 191,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 182,
                                                    end: 191,
                                                    as_str(): "Green: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 197,
                                                        end: 201,
                                                        as_str(): "Blue",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7272,
                                                ),
                                                initial_type_id: TypeId(
                                                    7271,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 203,
                                                    end: 205,
                                                    as_str(): "()",
                                                },
                                                tag: 2,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 197,
                                                    end: 205,
                                                    as_str(): "Blue: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 208,
                                            as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 182,
                                            end: 187,
                                            as_str(): "Green",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 1,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 1585,
                                        end: 1597,
                                        as_str(): "PrimaryColor",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 1599,
                                        end: 1604,
                                        as_str(): "Green",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 1599,
                                            end: 1604,
                                            as_str(): "Green",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7274,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1599,
                                    end: 1604,
                                    as_str(): "Green",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7274,
                            ),
                            type_ascription: TypeId(
                                7274,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1570,
                                    end: 1582,
                                    as_str(): "PrimaryColor",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 1553,
                    end: 1605,
                    as_str(): "let first_color: PrimaryColor = PrimaryColor::Green;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1614,
                                    end: 1618,
                                    as_str(): "test",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: FunctionApplication {
                                    call_path: CallPath {
                                        prefixes: [
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "core",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1633,
                                                    end: 1635,
                                                    as_str(): "==",
                                                },
                                                is_raw_ident: false,
                                            },
                                            BaseIdent {
                                                name_override_opt: Some(
                                                    "ops",
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1633,
                                                    end: 1635,
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
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 1633,
                                                end: 1635,
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 258,
                                                    end: 262,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1557,
                                                            end: 1568,
                                                            as_str(): "first_color",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1621,
                                                        end: 1632,
                                                        as_str(): "first_color",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7274,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1621,
                                                    end: 1632,
                                                    as_str(): "first_color",
                                                },
                                            },
                                        ),
                                        (
                                            BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 264,
                                                    end: 269,
                                                    as_str(): "other",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: EnumInstantiation {
                                                    enum_decl: TyEnumDeclaration {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0fb117fe0,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                ),
                                                                start: 150,
                                                                end: 162,
                                                                as_str(): "PrimaryColor",
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
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 169,
                                                                        end: 172,
                                                                        as_str(): "Red",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    7268,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    7267,
                                                                ),
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 174,
                                                                    end: 176,
                                                                    as_str(): "()",
                                                                },
                                                                tag: 0,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 169,
                                                                    end: 176,
                                                                    as_str(): "Red: ()",
                                                                },
                                                                attributes: {},
                                                            },
                                                            TyEnumVariant {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 182,
                                                                        end: 187,
                                                                        as_str(): "Green",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    7270,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    7269,
                                                                ),
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 189,
                                                                    end: 191,
                                                                    as_str(): "()",
                                                                },
                                                                tag: 1,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 182,
                                                                    end: 191,
                                                                    as_str(): "Green: ()",
                                                                },
                                                                attributes: {},
                                                            },
                                                            TyEnumVariant {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0fb117fe0,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                        ),
                                                                        start: 197,
                                                                        end: 201,
                                                                        as_str(): "Blue",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                type_id: TypeId(
                                                                    7272,
                                                                ),
                                                                initial_type_id: TypeId(
                                                                    7271,
                                                                ),
                                                                type_span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 203,
                                                                    end: 205,
                                                                    as_str(): "()",
                                                                },
                                                                tag: 2,
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0fb117fe0,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                                    ),
                                                                    start: 197,
                                                                    end: 205,
                                                                    as_str(): "Blue: ()",
                                                                },
                                                                attributes: {},
                                                            },
                                                        ],
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 145,
                                                            end: 208,
                                                            as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
                                                        },
                                                        visibility: Private,
                                                    },
                                                    variant_name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 182,
                                                            end: 187,
                                                            as_str(): "Green",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    tag: 1,
                                                    contents: None,
                                                    enum_instantiation_span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1636,
                                                        end: 1648,
                                                        as_str(): "PrimaryColor",
                                                    },
                                                    variant_instantiation_span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1650,
                                                        end: 1655,
                                                        as_str(): "Green",
                                                    },
                                                    type_binding: TypeBinding {
                                                        inner: (),
                                                        type_arguments: [],
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1650,
                                                            end: 1655,
                                                            as_str(): "Green",
                                                        },
                                                    },
                                                },
                                                return_type: TypeId(
                                                    7274,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1650,
                                                    end: 1655,
                                                    as_str(): "Green",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        569,
                                        Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 252,
                                            end: 387,
                                            as_str(): "fn eq(self, other: Self) -> bool {\n        asm(r1: self, r2: other, r3) {\n            eq r3 r1 r2;\n            r3: bool\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 1633,
                                                end: 1635,
                                                as_str(): "==",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    71,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1621,
                                    end: 1655,
                                    as_str(): "first_color == PrimaryColor::Green",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                71,
                            ),
                            type_ascription: TypeId(
                                7412,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 1610,
                    end: 1656,
                    as_str(): "let test = first_color == PrimaryColor::Green;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1745,
                                    end: 1748,
                                    as_str(): "rgb",
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
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 1768,
                                                end: 1771,
                                                as_str(): "rgb",
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 834,
                                                    end: 838,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1557,
                                                            end: 1568,
                                                            as_str(): "first_color",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1756,
                                                        end: 1767,
                                                        as_str(): "first_color",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7274,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1756,
                                                    end: 1767,
                                                    as_str(): "first_color",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        570,
                                        Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 827,
                                            end: 1526,
                                            as_str(): "fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 1768,
                                                end: 1771,
                                                as_str(): "rgb",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7255,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1756,
                                    end: 1773,
                                    as_str(): "first_color.rgb()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7255,
                            ),
                            type_ascription: TypeId(
                                7255,
                            ),
                            type_ascription_span: Some(
                                Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1750,
                                    end: 1753,
                                    as_str(): "Rgb",
                                },
                            ),
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 1741,
                    end: 1774,
                    as_str(): "let rgb: Rgb = first_color.rgb();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1854,
                                    end: 1866,
                                    as_str(): "second_color",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 150,
                                                end: 162,
                                                as_str(): "PrimaryColor",
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 172,
                                                        as_str(): "Red",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7268,
                                                ),
                                                initial_type_id: TypeId(
                                                    7267,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 176,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 176,
                                                    as_str(): "Red: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 182,
                                                        end: 187,
                                                        as_str(): "Green",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7270,
                                                ),
                                                initial_type_id: TypeId(
                                                    7269,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 189,
                                                    end: 191,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 182,
                                                    end: 191,
                                                    as_str(): "Green: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 197,
                                                        end: 201,
                                                        as_str(): "Blue",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7272,
                                                ),
                                                initial_type_id: TypeId(
                                                    7271,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 203,
                                                    end: 205,
                                                    as_str(): "()",
                                                },
                                                tag: 2,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 197,
                                                    end: 205,
                                                    as_str(): "Blue: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 208,
                                            as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 201,
                                            as_str(): "Blue",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 2,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 1869,
                                        end: 1881,
                                        as_str(): "PrimaryColor",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 1883,
                                        end: 1887,
                                        as_str(): "Blue",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 1883,
                                            end: 1887,
                                            as_str(): "Blue",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7274,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1883,
                                    end: 1887,
                                    as_str(): "Blue",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7274,
                            ),
                            type_ascription: TypeId(
                                7417,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 1850,
                    end: 1888,
                    as_str(): "let second_color = PrimaryColor::Blue;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1897,
                                    end: 1907,
                                    as_str(): "second_rgb",
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
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 1923,
                                                end: 1926,
                                                as_str(): "rgb",
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 834,
                                                    end: 838,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1854,
                                                            end: 1866,
                                                            as_str(): "second_color",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1910,
                                                        end: 1922,
                                                        as_str(): "second_color",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7274,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1910,
                                                    end: 1922,
                                                    as_str(): "second_color",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        572,
                                        Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 827,
                                            end: 1526,
                                            as_str(): "fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 1923,
                                                end: 1926,
                                                as_str(): "rgb",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7255,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1910,
                                    end: 1928,
                                    as_str(): "second_color.rgb()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7255,
                            ),
                            type_ascription: TypeId(
                                7418,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 1893,
                    end: 1929,
                    as_str(): "let second_rgb = second_color.rgb();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1938,
                                    end: 1950,
                                    as_str(): "second_color",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 150,
                                                end: 162,
                                                as_str(): "PrimaryColor",
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 172,
                                                        as_str(): "Red",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7268,
                                                ),
                                                initial_type_id: TypeId(
                                                    7267,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 176,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 176,
                                                    as_str(): "Red: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 182,
                                                        end: 187,
                                                        as_str(): "Green",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7270,
                                                ),
                                                initial_type_id: TypeId(
                                                    7269,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 189,
                                                    end: 191,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 182,
                                                    end: 191,
                                                    as_str(): "Green: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 197,
                                                        end: 201,
                                                        as_str(): "Blue",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7272,
                                                ),
                                                initial_type_id: TypeId(
                                                    7271,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 203,
                                                    end: 205,
                                                    as_str(): "()",
                                                },
                                                tag: 2,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 197,
                                                    end: 205,
                                                    as_str(): "Blue: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 208,
                                            as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 201,
                                            as_str(): "Blue",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 2,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 1953,
                                        end: 1965,
                                        as_str(): "PrimaryColor",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 1967,
                                        end: 1971,
                                        as_str(): "Blue",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 1967,
                                            end: 1971,
                                            as_str(): "Blue",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7274,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1967,
                                    end: 1971,
                                    as_str(): "Blue",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7274,
                            ),
                            type_ascription: TypeId(
                                7420,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 1934,
                    end: 1972,
                    as_str(): "let second_color = PrimaryColor::Blue;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1981,
                                    end: 1991,
                                    as_str(): "second_rgb",
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
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 2007,
                                                end: 2010,
                                                as_str(): "rgb",
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 834,
                                                    end: 838,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 1938,
                                                            end: 1950,
                                                            as_str(): "second_color",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 1994,
                                                        end: 2006,
                                                        as_str(): "second_color",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7274,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 1994,
                                                    end: 2006,
                                                    as_str(): "second_color",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        574,
                                        Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 827,
                                            end: 1526,
                                            as_str(): "fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 2007,
                                                end: 2010,
                                                as_str(): "rgb",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7255,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 1994,
                                    end: 2012,
                                    as_str(): "second_color.rgb()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7255,
                            ),
                            type_ascription: TypeId(
                                7421,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 1977,
                    end: 2013,
                    as_str(): "let second_rgb = second_color.rgb();",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 2022,
                                    end: 2034,
                                    as_str(): "second_color",
                                },
                                is_raw_ident: false,
                            },
                            body: TyExpression {
                                expression: EnumInstantiation {
                                    enum_decl: TyEnumDeclaration {
                                        name: BaseIdent {
                                            name_override_opt: None,
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 150,
                                                end: 162,
                                                as_str(): "PrimaryColor",
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
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 169,
                                                        end: 172,
                                                        as_str(): "Red",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7268,
                                                ),
                                                initial_type_id: TypeId(
                                                    7267,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 174,
                                                    end: 176,
                                                    as_str(): "()",
                                                },
                                                tag: 0,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 169,
                                                    end: 176,
                                                    as_str(): "Red: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 182,
                                                        end: 187,
                                                        as_str(): "Green",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7270,
                                                ),
                                                initial_type_id: TypeId(
                                                    7269,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 189,
                                                    end: 191,
                                                    as_str(): "()",
                                                },
                                                tag: 1,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 182,
                                                    end: 191,
                                                    as_str(): "Green: ()",
                                                },
                                                attributes: {},
                                            },
                                            TyEnumVariant {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 197,
                                                        end: 201,
                                                        as_str(): "Blue",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                type_id: TypeId(
                                                    7272,
                                                ),
                                                initial_type_id: TypeId(
                                                    7271,
                                                ),
                                                type_span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 203,
                                                    end: 205,
                                                    as_str(): "()",
                                                },
                                                tag: 2,
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 197,
                                                    end: 205,
                                                    as_str(): "Blue: ()",
                                                },
                                                attributes: {},
                                            },
                                        ],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 145,
                                            end: 208,
                                            as_str(): "enum PrimaryColor {\n    Red: (),\n    Green: (),\n    Blue: (),\n}",
                                        },
                                        visibility: Private,
                                    },
                                    variant_name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 197,
                                            end: 201,
                                            as_str(): "Blue",
                                        },
                                        is_raw_ident: false,
                                    },
                                    tag: 2,
                                    contents: None,
                                    enum_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 2037,
                                        end: 2049,
                                        as_str(): "PrimaryColor",
                                    },
                                    variant_instantiation_span: Span {
                                        src (ptr): 0x00007fe0fb117fe0,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                        ),
                                        start: 2051,
                                        end: 2055,
                                        as_str(): "Blue",
                                    },
                                    type_binding: TypeBinding {
                                        inner: (),
                                        type_arguments: [],
                                        span: Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 2051,
                                            end: 2055,
                                            as_str(): "Blue",
                                        },
                                    },
                                },
                                return_type: TypeId(
                                    7274,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 2051,
                                    end: 2055,
                                    as_str(): "Blue",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7274,
                            ),
                            type_ascription: TypeId(
                                7423,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 2018,
                    end: 2056,
                    as_str(): "let second_color = PrimaryColor::Blue;",
                },
            },
            TyAstNode {
                content: Declaration(
                    VariableDeclaration(
                        TyVariableDeclaration {
                            name: BaseIdent {
                                name_override_opt: None,
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 2065,
                                    end: 2075,
                                    as_str(): "second_rgb",
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
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 2091,
                                                end: 2094,
                                                as_str(): "rgb",
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
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 834,
                                                    end: 838,
                                                    as_str(): "self",
                                                },
                                                is_raw_ident: false,
                                            },
                                            TyExpression {
                                                expression: VariableExpression {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0fb117fe0,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                            ),
                                                            start: 2022,
                                                            end: 2034,
                                                            as_str(): "second_color",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0fb117fe0,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                        ),
                                                        start: 2078,
                                                        end: 2090,
                                                        as_str(): "second_color",
                                                    },
                                                    mutability: Immutable,
                                                },
                                                return_type: TypeId(
                                                    7274,
                                                ),
                                                span: Span {
                                                    src (ptr): 0x00007fe0fb117fe0,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                    ),
                                                    start: 2078,
                                                    end: 2090,
                                                    as_str(): "second_color",
                                                },
                                            },
                                        ),
                                    ],
                                    function_decl_id: DeclId(
                                        576,
                                        Span {
                                            src (ptr): 0x00007fe0fb117fe0,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                            ),
                                            start: 827,
                                            end: 1526,
                                            as_str(): "fn rgb(self) -> Rgb {\n        if self == PrimaryColor::Red {\n            Rgb {\n                red: 255,\n                blue: 0,\n                green: 0,\n            }\n        } else if self == PrimaryColor::Green {\n            Rgb {\n                red: 0,\n                blue: 0,\n                green: 255,\n            }\n        } else if self == PrimaryColor::Blue {\n            Rgb {\n                red: 0,\n                blue: 255,\n                green: 0,\n            }\n        }\n        // TODO remove this else when exhaustive ifs are checked for\n        else {\n            Rgb {\n                red: 0,\n                green: 0,\n                blue: 0,\n            }\n        }\n    }",
                                        },
                                    ),
                                    self_state_idx: None,
                                    selector: None,
                                    type_binding: Some(
                                        TypeBinding {
                                            inner: (),
                                            type_arguments: [],
                                            span: Span {
                                                src (ptr): 0x00007fe0fb117fe0,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                                ),
                                                start: 2091,
                                                end: 2094,
                                                as_str(): "rgb",
                                            },
                                        },
                                    ),
                                },
                                return_type: TypeId(
                                    7255,
                                ),
                                span: Span {
                                    src (ptr): 0x00007fe0fb117fe0,
                                    path: Some(
                                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                                    ),
                                    start: 2078,
                                    end: 2096,
                                    as_str(): "second_color.rgb()",
                                },
                            },
                            mutability: Immutable,
                            return_type: TypeId(
                                7255,
                            ),
                            type_ascription: TypeId(
                                7424,
                            ),
                            type_ascription_span: None,
                        },
                    ),
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 2061,
                    end: 2097,
                    as_str(): "let second_rgb = second_color.rgb();",
                },
            },
            TyAstNode {
                content: ImplicitReturnExpression(
                    TyExpression {
                        expression: Literal(
                            U32(
                                10,
                            ),
                        ),
                        return_type: TypeId(
                            33,
                        ),
                        span: Span {
                            src (ptr): 0x00007fe0fb117fe0,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                            ),
                            start: 2102,
                            end: 2107,
                            as_str(): "10u32",
                        },
                    },
                ),
                span: Span {
                    src (ptr): 0x00007fe0fb117fe0,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
                    ),
                    start: 2102,
                    end: 2107,
                    as_str(): "10u32",
                },
            },
        ],
    },
    parameters: [],
    implementing_type: None,
    span: Span {
        src (ptr): 0x00007fe0fb117fe0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
        ),
        start: 1530,
        end: 2109,
        as_str(): "fn main() -> u32 {\n    let first_color: PrimaryColor = PrimaryColor::Green;\n    let test = first_color == PrimaryColor::Green;\n    // Specifically, when we call methods in the below way, `self` is undefined\n    let rgb: Rgb = first_color.rgb();\n    // now, going to test the register pool by using over 48 registers\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    let second_color = PrimaryColor::Blue;\n    let second_rgb = second_color.rgb();\n    10u32\n}",
    },
    attributes: {},
    return_type: TypeId(
        33,
    ),
    initial_return_type: TypeId(
        33,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe0fb117fe0,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRqu0s8R/if_elseif_enum/src/main.sw",
        ),
        start: 1543,
        end: 1546,
        as_str(): "u32",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

