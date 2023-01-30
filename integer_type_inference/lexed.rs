Some(
    LexedProgram {
        kind: Script,
        root: LexedModule {
            tree: Module {
                kind: Script {
                    script_token: ScriptToken {
                        span: Span {
                            src (ptr): 0x00007fe0dc169290,
                            path: Some(
                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                            ),
                            start: 0,
                            end: 6,
                            as_str(): "script",
                        },
                    },
                },
                semicolon_token: SemicolonToken {
                    span: Span {
                        src (ptr): 0x00007fe0dc169290,
                        path: Some(
                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                        ),
                        start: 6,
                        end: 7,
                        as_str(): ";",
                    },
                },
                items: [
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 30,
                                        end: 35,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 36,
                                        end: 38,
                                        as_str(): "X1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 38,
                                                end: 39,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 40,
                                                            end: 42,
                                                            as_str(): "u8",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 43,
                                        end: 44,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 45,
                                                end: 46,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U8,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 46,
                                                        end: 48,
                                                        as_str(): "u8",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 48,
                                        end: 49,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 50,
                                        end: 55,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 56,
                                        end: 58,
                                        as_str(): "X2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 58,
                                                end: 59,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 60,
                                                            end: 62,
                                                            as_str(): "u8",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 63,
                                        end: 64,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 65,
                                                end: 66,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U16,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 66,
                                                        end: 69,
                                                        as_str(): "u16",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 69,
                                        end: 70,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 71,
                                        end: 76,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 77,
                                        end: 79,
                                        as_str(): "X3",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 79,
                                                end: 80,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 81,
                                                            end: 83,
                                                            as_str(): "u8",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 84,
                                        end: 85,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 86,
                                                end: 87,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U32,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 87,
                                                        end: 90,
                                                        as_str(): "u32",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 90,
                                        end: 91,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 92,
                                        end: 97,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 98,
                                        end: 100,
                                        as_str(): "X4",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 100,
                                                end: 101,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 102,
                                                            end: 104,
                                                            as_str(): "u8",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 105,
                                        end: 106,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 107,
                                                end: 108,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U64,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 108,
                                                        end: 111,
                                                        as_str(): "u64",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 111,
                                        end: 112,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 113,
                                        end: 118,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 119,
                                        end: 121,
                                        as_str(): "X5",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 121,
                                                end: 122,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 123,
                                                            end: 125,
                                                            as_str(): "u8",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 126,
                                        end: 127,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 128,
                                                end: 129,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 129,
                                        end: 130,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 132,
                                        end: 137,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 138,
                                        end: 140,
                                        as_str(): "Y1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 140,
                                                end: 141,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 142,
                                                            end: 145,
                                                            as_str(): "u16",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 146,
                                        end: 147,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 148,
                                                end: 149,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U8,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 149,
                                                        end: 151,
                                                        as_str(): "u8",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 151,
                                        end: 152,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 153,
                                        end: 158,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 159,
                                        end: 161,
                                        as_str(): "Y2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 161,
                                                end: 162,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 163,
                                                            end: 166,
                                                            as_str(): "u16",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 167,
                                        end: 168,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 169,
                                                end: 170,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U16,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 170,
                                                        end: 173,
                                                        as_str(): "u16",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 173,
                                        end: 174,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 175,
                                        end: 180,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 181,
                                        end: 183,
                                        as_str(): "Y3",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 183,
                                                end: 184,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 185,
                                                            end: 188,
                                                            as_str(): "u16",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 189,
                                        end: 190,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 191,
                                                end: 192,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U32,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 192,
                                                        end: 195,
                                                        as_str(): "u32",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 195,
                                        end: 196,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 197,
                                        end: 202,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 203,
                                        end: 205,
                                        as_str(): "Y4",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 205,
                                                end: 206,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 207,
                                                            end: 210,
                                                            as_str(): "u16",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 211,
                                        end: 212,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 213,
                                                end: 214,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U64,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 214,
                                                        end: 217,
                                                        as_str(): "u64",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 217,
                                        end: 218,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 219,
                                        end: 224,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 225,
                                        end: 227,
                                        as_str(): "Y5",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 227,
                                                end: 228,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 229,
                                                            end: 232,
                                                            as_str(): "u16",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 233,
                                        end: 234,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 235,
                                                end: 236,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 236,
                                        end: 237,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 239,
                                        end: 244,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 245,
                                        end: 247,
                                        as_str(): "Z1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 247,
                                                end: 248,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 249,
                                                            end: 252,
                                                            as_str(): "u32",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 253,
                                        end: 254,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 255,
                                                end: 256,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U8,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 256,
                                                        end: 258,
                                                        as_str(): "u8",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 258,
                                        end: 259,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 260,
                                        end: 265,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 266,
                                        end: 268,
                                        as_str(): "Z2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 268,
                                                end: 269,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 270,
                                                            end: 273,
                                                            as_str(): "u32",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 274,
                                        end: 275,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 276,
                                                end: 277,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U16,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 277,
                                                        end: 280,
                                                        as_str(): "u16",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 280,
                                        end: 281,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 282,
                                        end: 287,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 288,
                                        end: 290,
                                        as_str(): "Z3",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 290,
                                                end: 291,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 292,
                                                            end: 295,
                                                            as_str(): "u32",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 296,
                                        end: 297,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 298,
                                                end: 299,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U32,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 299,
                                                        end: 302,
                                                        as_str(): "u32",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 302,
                                        end: 303,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 304,
                                        end: 309,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 310,
                                        end: 312,
                                        as_str(): "Z4",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 312,
                                                end: 313,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 314,
                                                            end: 317,
                                                            as_str(): "u32",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 318,
                                        end: 319,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 320,
                                                end: 321,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U64,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 321,
                                                        end: 324,
                                                        as_str(): "u64",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 324,
                                        end: 325,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 326,
                                        end: 331,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 332,
                                        end: 334,
                                        as_str(): "Z5",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 334,
                                                end: 335,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 336,
                                                            end: 339,
                                                            as_str(): "u32",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 340,
                                        end: 341,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 342,
                                                end: 343,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 343,
                                        end: 344,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 346,
                                        end: 351,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 352,
                                        end: 354,
                                        as_str(): "W1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 354,
                                                end: 355,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 356,
                                                            end: 359,
                                                            as_str(): "u64",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 360,
                                        end: 361,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 362,
                                                end: 363,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U8,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 363,
                                                        end: 365,
                                                        as_str(): "u8",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 365,
                                        end: 366,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 367,
                                        end: 372,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 373,
                                        end: 375,
                                        as_str(): "W2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 375,
                                                end: 376,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 377,
                                                            end: 380,
                                                            as_str(): "u64",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 381,
                                        end: 382,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 383,
                                                end: 384,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U16,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 384,
                                                        end: 387,
                                                        as_str(): "u16",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 387,
                                        end: 388,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 389,
                                        end: 394,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 395,
                                        end: 397,
                                        as_str(): "W3",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 397,
                                                end: 398,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 399,
                                                            end: 402,
                                                            as_str(): "u64",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 403,
                                        end: 404,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 405,
                                                end: 406,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U32,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 406,
                                                        end: 409,
                                                        as_str(): "u32",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 409,
                                        end: 410,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 411,
                                        end: 416,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 417,
                                        end: 419,
                                        as_str(): "W4",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 419,
                                                end: 420,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 421,
                                                            end: 424,
                                                            as_str(): "u64",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 425,
                                        end: 426,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 427,
                                                end: 428,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U64,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 428,
                                                        end: 431,
                                                        as_str(): "u64",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 431,
                                        end: 432,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 433,
                                        end: 438,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 439,
                                        end: 441,
                                        as_str(): "W5",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: Some(
                                    (
                                        ColonToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 441,
                                                end: 442,
                                                as_str(): ":",
                                            },
                                        },
                                        Path(
                                            PathType {
                                                root_opt: None,
                                                prefix: PathTypeSegment {
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 443,
                                                            end: 446,
                                                            as_str(): "u64",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics_opt: None,
                                                },
                                                suffix: [],
                                            },
                                        ),
                                    ),
                                ),
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 447,
                                        end: 448,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 449,
                                                end: 450,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 450,
                                        end: 451,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 453,
                                        end: 458,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 459,
                                        end: 461,
                                        as_str(): "V1",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 462,
                                        end: 463,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 464,
                                                end: 465,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U8,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 465,
                                                        end: 467,
                                                        as_str(): "u8",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 467,
                                        end: 468,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 469,
                                        end: 474,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 475,
                                        end: 477,
                                        as_str(): "V2",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 478,
                                        end: 479,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 480,
                                                end: 481,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U16,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 481,
                                                        end: 484,
                                                        as_str(): "u16",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 484,
                                        end: 485,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 486,
                                        end: 491,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 492,
                                        end: 494,
                                        as_str(): "V3",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 495,
                                        end: 496,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 497,
                                                end: 498,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U32,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 498,
                                                        end: 501,
                                                        as_str(): "u32",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 501,
                                        end: 502,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 503,
                                        end: 508,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 509,
                                        end: 511,
                                        as_str(): "V4",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 512,
                                        end: 513,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 514,
                                                end: 515,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: Some(
                                                (
                                                    U64,
                                                    Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 515,
                                                        end: 518,
                                                        as_str(): "u64",
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 518,
                                        end: 519,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Const(
                            ItemConst {
                                visibility: None,
                                const_token: ConstToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 520,
                                        end: 525,
                                        as_str(): "const",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 526,
                                        end: 528,
                                        as_str(): "V5",
                                    },
                                    is_raw_ident: false,
                                },
                                ty_opt: None,
                                eq_token: EqToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 529,
                                        end: 530,
                                        as_str(): "=",
                                    },
                                },
                                expr: Literal(
                                    Int(
                                        LitInt {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 531,
                                                end: 532,
                                                as_str(): "4",
                                            },
                                            parsed: 4,
                                            ty_opt: None,
                                        },
                                    ),
                                ),
                                semicolon_token: SemicolonToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 532,
                                        end: 533,
                                        as_str(): ";",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 585,
                                        end: 590,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 591,
                                        end: 596,
                                        as_str(): "FooU8",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 603,
                                                            end: 605,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 606,
                                                            end: 612,
                                                            as_str(): "foo_u8",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 613,
                                                                    end: 617,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 612,
                                                            end: 618,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 618,
                                                    end: 619,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 597,
                                        end: 621,
                                        as_str(): "{\n    fn foo_u8(self);\n}",
                                    },
                                },
                                trait_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 623,
                                        end: 628,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 629,
                                        end: 635,
                                        as_str(): "FooU16",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 642,
                                                            end: 644,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 645,
                                                            end: 652,
                                                            as_str(): "foo_u16",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 653,
                                                                    end: 657,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 652,
                                                            end: 658,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 658,
                                                    end: 659,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 636,
                                        end: 661,
                                        as_str(): "{\n    fn foo_u16(self);\n}",
                                    },
                                },
                                trait_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 663,
                                        end: 668,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 669,
                                        end: 675,
                                        as_str(): "FooU32",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 682,
                                                            end: 684,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 685,
                                                            end: 692,
                                                            as_str(): "foo_u32",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 693,
                                                                    end: 697,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 692,
                                                            end: 698,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 698,
                                                    end: 699,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 676,
                                        end: 701,
                                        as_str(): "{\n    fn foo_u32(self);\n}",
                                    },
                                },
                                trait_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Trait(
                            ItemTrait {
                                visibility: None,
                                trait_token: TraitToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 703,
                                        end: 708,
                                        as_str(): "trait",
                                    },
                                },
                                name: BaseIdent {
                                    name_override_opt: None,
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 709,
                                        end: 715,
                                        as_str(): "FooU64",
                                    },
                                    is_raw_ident: false,
                                },
                                generics: None,
                                where_clause_opt: None,
                                super_traits: None,
                                trait_items: Braces {
                                    inner: [
                                        (
                                            Annotated {
                                                attribute_list: [],
                                                value: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 722,
                                                            end: 724,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 725,
                                                            end: 732,
                                                            as_str(): "foo_u64",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 733,
                                                                    end: 737,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 732,
                                                            end: 738,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                            },
                                            SemicolonToken {
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 738,
                                                    end: 739,
                                                    as_str(): ";",
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 716,
                                        end: 741,
                                        as_str(): "{\n    fn foo_u64(self);\n}",
                                    },
                                },
                                trait_defs_opt: None,
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 761,
                                        end: 765,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: Some(
                                    (
                                        PathType {
                                            root_opt: None,
                                            prefix: PathTypeSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 766,
                                                        end: 771,
                                                        as_str(): "FooU8",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 772,
                                                end: 775,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 776,
                                                    end: 778,
                                                    as_str(): "u8",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 785,
                                                            end: 787,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 788,
                                                            end: 794,
                                                            as_str(): "foo_u8",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 795,
                                                                    end: 799,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 794,
                                                            end: 800,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 801,
                                                        end: 803,
                                                        as_str(): "{}",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 779,
                                        end: 805,
                                        as_str(): "{\n    fn foo_u8(self) {}\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 807,
                                        end: 811,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: Some(
                                    (
                                        PathType {
                                            root_opt: None,
                                            prefix: PathTypeSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 812,
                                                        end: 818,
                                                        as_str(): "FooU16",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 819,
                                                end: 822,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 823,
                                                    end: 826,
                                                    as_str(): "u16",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 833,
                                                            end: 835,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 836,
                                                            end: 843,
                                                            as_str(): "foo_u16",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 844,
                                                                    end: 848,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 843,
                                                            end: 849,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 850,
                                                        end: 852,
                                                        as_str(): "{}",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 827,
                                        end: 854,
                                        as_str(): "{\n    fn foo_u16(self) {}\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 856,
                                        end: 860,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: Some(
                                    (
                                        PathType {
                                            root_opt: None,
                                            prefix: PathTypeSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 861,
                                                        end: 867,
                                                        as_str(): "FooU32",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 868,
                                                end: 871,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 872,
                                                    end: 875,
                                                    as_str(): "u32",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 882,
                                                            end: 884,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 885,
                                                            end: 892,
                                                            as_str(): "foo_u32",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 893,
                                                                    end: 897,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 892,
                                                            end: 898,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 899,
                                                        end: 901,
                                                        as_str(): "{}",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 876,
                                        end: 903,
                                        as_str(): "{\n    fn foo_u32(self) {}\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Impl(
                            ItemImpl {
                                impl_token: ImplToken {
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 905,
                                        end: 909,
                                        as_str(): "impl",
                                    },
                                },
                                generic_params_opt: None,
                                trait_opt: Some(
                                    (
                                        PathType {
                                            root_opt: None,
                                            prefix: PathTypeSegment {
                                                name: BaseIdent {
                                                    name_override_opt: None,
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 910,
                                                        end: 916,
                                                        as_str(): "FooU64",
                                                    },
                                                    is_raw_ident: false,
                                                },
                                                generics_opt: None,
                                            },
                                            suffix: [],
                                        },
                                        ForToken {
                                            span: Span {
                                                src (ptr): 0x00007fe0dc169290,
                                                path: Some(
                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                ),
                                                start: 917,
                                                end: 920,
                                                as_str(): "for",
                                            },
                                        },
                                    ),
                                ),
                                ty: Path(
                                    PathType {
                                        root_opt: None,
                                        prefix: PathTypeSegment {
                                            name: BaseIdent {
                                                name_override_opt: None,
                                                span: Span {
                                                    src (ptr): 0x00007fe0dc169290,
                                                    path: Some(
                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                    ),
                                                    start: 921,
                                                    end: 924,
                                                    as_str(): "u64",
                                                },
                                                is_raw_ident: false,
                                            },
                                            generics_opt: None,
                                        },
                                        suffix: [],
                                    },
                                ),
                                where_clause_opt: None,
                                contents: Braces {
                                    inner: [
                                        Annotated {
                                            attribute_list: [],
                                            value: ItemFn {
                                                fn_signature: FnSignature {
                                                    visibility: None,
                                                    fn_token: FnToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 931,
                                                            end: 933,
                                                            as_str(): "fn",
                                                        },
                                                    },
                                                    name: BaseIdent {
                                                        name_override_opt: None,
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 934,
                                                            end: 941,
                                                            as_str(): "foo_u64",
                                                        },
                                                        is_raw_ident: false,
                                                    },
                                                    generics: None,
                                                    arguments: Parens {
                                                        inner: NonStatic {
                                                            self_token: SelfToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 942,
                                                                    end: 946,
                                                                    as_str(): "self",
                                                                },
                                                            },
                                                            ref_self: None,
                                                            mutable_self: None,
                                                            args_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 941,
                                                            end: 947,
                                                            as_str(): "(self)",
                                                        },
                                                    },
                                                    return_type_opt: None,
                                                    where_clause_opt: None,
                                                },
                                                body: Braces {
                                                    inner: CodeBlockContents {
                                                        statements: [],
                                                        final_expr_opt: None,
                                                    },
                                                    span: Span {
                                                        src (ptr): 0x00007fe0dc169290,
                                                        path: Some(
                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                        ),
                                                        start: 948,
                                                        end: 950,
                                                        as_str(): "{}",
                                                    },
                                                },
                                            },
                                        },
                                    ],
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 925,
                                        end: 952,
                                        as_str(): "{\n    fn foo_u64(self) {}\n}",
                                    },
                                },
                            },
                        ),
                    },
                    Annotated {
                        attribute_list: [],
                        value: Fn(
                            ItemFn {
                                fn_signature: FnSignature {
                                    visibility: None,
                                    fn_token: FnToken {
                                        span: Span {
                                            src (ptr): 0x00007fe0dc169290,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                            ),
                                            start: 954,
                                            end: 956,
                                            as_str(): "fn",
                                        },
                                    },
                                    name: BaseIdent {
                                        name_override_opt: None,
                                        span: Span {
                                            src (ptr): 0x00007fe0dc169290,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                            ),
                                            start: 957,
                                            end: 961,
                                            as_str(): "main",
                                        },
                                        is_raw_ident: false,
                                    },
                                    generics: None,
                                    arguments: Parens {
                                        inner: Static(
                                            Punctuated {
                                                value_separator_pairs: [],
                                                final_value_opt: None,
                                            },
                                        ),
                                        span: Span {
                                            src (ptr): 0x00007fe0dc169290,
                                            path: Some(
                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                            ),
                                            start: 961,
                                            end: 963,
                                            as_str(): "()",
                                        },
                                    },
                                    return_type_opt: None,
                                    where_clause_opt: None,
                                },
                                body: Braces {
                                    inner: CodeBlockContents {
                                        statements: [
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1040,
                                                                        end: 1042,
                                                                        as_str(): "X1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1042,
                                                            end: 1043,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1043,
                                                                end: 1049,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1049,
                                                            end: 1051,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1051,
                                                            end: 1052,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1057,
                                                                        end: 1059,
                                                                        as_str(): "X2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1059,
                                                            end: 1060,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1060,
                                                                end: 1066,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1066,
                                                            end: 1068,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1068,
                                                            end: 1069,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1074,
                                                                        end: 1076,
                                                                        as_str(): "X3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1076,
                                                            end: 1077,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1077,
                                                                end: 1083,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1083,
                                                            end: 1085,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1085,
                                                            end: 1086,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1091,
                                                                        end: 1093,
                                                                        as_str(): "X4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1093,
                                                            end: 1094,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1094,
                                                                end: 1100,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1100,
                                                            end: 1102,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1102,
                                                            end: 1103,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1108,
                                                                        end: 1110,
                                                                        as_str(): "X5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1110,
                                                            end: 1111,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1111,
                                                                end: 1117,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1117,
                                                            end: 1119,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1119,
                                                            end: 1120,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1126,
                                                                        end: 1128,
                                                                        as_str(): "Y1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1128,
                                                            end: 1129,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1129,
                                                                end: 1136,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1136,
                                                            end: 1138,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1138,
                                                            end: 1139,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1144,
                                                                        end: 1146,
                                                                        as_str(): "Y2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1146,
                                                            end: 1147,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1147,
                                                                end: 1154,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1154,
                                                            end: 1156,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1156,
                                                            end: 1157,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1162,
                                                                        end: 1164,
                                                                        as_str(): "Y3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1164,
                                                            end: 1165,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1165,
                                                                end: 1172,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1172,
                                                            end: 1174,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1174,
                                                            end: 1175,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1180,
                                                                        end: 1182,
                                                                        as_str(): "Y4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1182,
                                                            end: 1183,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1183,
                                                                end: 1190,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1190,
                                                            end: 1192,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1192,
                                                            end: 1193,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1198,
                                                                        end: 1200,
                                                                        as_str(): "Y5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1200,
                                                            end: 1201,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1201,
                                                                end: 1208,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1208,
                                                            end: 1210,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1210,
                                                            end: 1211,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1217,
                                                                        end: 1219,
                                                                        as_str(): "Z1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1219,
                                                            end: 1220,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1220,
                                                                end: 1227,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1227,
                                                            end: 1229,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1229,
                                                            end: 1230,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1235,
                                                                        end: 1237,
                                                                        as_str(): "Z2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1237,
                                                            end: 1238,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1238,
                                                                end: 1245,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1245,
                                                            end: 1247,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1247,
                                                            end: 1248,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1253,
                                                                        end: 1255,
                                                                        as_str(): "Z3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1255,
                                                            end: 1256,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1256,
                                                                end: 1263,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1263,
                                                            end: 1265,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1265,
                                                            end: 1266,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1271,
                                                                        end: 1273,
                                                                        as_str(): "Z4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1273,
                                                            end: 1274,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1274,
                                                                end: 1281,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1281,
                                                            end: 1283,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1283,
                                                            end: 1284,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1289,
                                                                        end: 1291,
                                                                        as_str(): "Z5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1291,
                                                            end: 1292,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1292,
                                                                end: 1299,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1299,
                                                            end: 1301,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1301,
                                                            end: 1302,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1308,
                                                                        end: 1310,
                                                                        as_str(): "W1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1310,
                                                            end: 1311,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1311,
                                                                end: 1318,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1318,
                                                            end: 1320,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1320,
                                                            end: 1321,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1326,
                                                                        end: 1328,
                                                                        as_str(): "W2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1328,
                                                            end: 1329,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1329,
                                                                end: 1336,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1336,
                                                            end: 1338,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1338,
                                                            end: 1339,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1344,
                                                                        end: 1346,
                                                                        as_str(): "W3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1346,
                                                            end: 1347,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1347,
                                                                end: 1354,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1354,
                                                            end: 1356,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1356,
                                                            end: 1357,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1362,
                                                                        end: 1364,
                                                                        as_str(): "W4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1364,
                                                            end: 1365,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1365,
                                                                end: 1372,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1372,
                                                            end: 1374,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1374,
                                                            end: 1375,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1380,
                                                                        end: 1382,
                                                                        as_str(): "W5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1382,
                                                            end: 1383,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1383,
                                                                end: 1390,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1390,
                                                            end: 1392,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1392,
                                                            end: 1393,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1399,
                                                                        end: 1401,
                                                                        as_str(): "V1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1401,
                                                            end: 1402,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1402,
                                                                end: 1408,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1408,
                                                            end: 1410,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1410,
                                                            end: 1411,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1416,
                                                                        end: 1418,
                                                                        as_str(): "V2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1418,
                                                            end: 1419,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1419,
                                                                end: 1426,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1426,
                                                            end: 1428,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1428,
                                                            end: 1429,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1434,
                                                                        end: 1436,
                                                                        as_str(): "V3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1436,
                                                            end: 1437,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1437,
                                                                end: 1444,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1444,
                                                            end: 1446,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1446,
                                                            end: 1447,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1452,
                                                                        end: 1454,
                                                                        as_str(): "V4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1454,
                                                            end: 1455,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1455,
                                                                end: 1462,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1462,
                                                            end: 1464,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1464,
                                                            end: 1465,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 1470,
                                                                        end: 1472,
                                                                        as_str(): "V5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1472,
                                                            end: 1473,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1473,
                                                                end: 1480,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1480,
                                                            end: 1482,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1482,
                                                            end: 1483,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1559,
                                                            end: 1562,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1563,
                                                                end: 1565,
                                                                as_str(): "x1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1565,
                                                                    end: 1566,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1567,
                                                                                end: 1569,
                                                                                as_str(): "u8",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1570,
                                                            end: 1571,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1572,
                                                                    end: 1573,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U8,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1573,
                                                                            end: 1575,
                                                                            as_str(): "u8",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1575,
                                                            end: 1576,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1581,
                                                            end: 1584,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1585,
                                                                end: 1587,
                                                                as_str(): "x2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1587,
                                                                    end: 1588,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1589,
                                                                                end: 1591,
                                                                                as_str(): "u8",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1592,
                                                            end: 1593,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1594,
                                                                    end: 1595,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U16,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1595,
                                                                            end: 1598,
                                                                            as_str(): "u16",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1598,
                                                            end: 1599,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1604,
                                                            end: 1607,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1608,
                                                                end: 1610,
                                                                as_str(): "x3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1610,
                                                                    end: 1611,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1612,
                                                                                end: 1614,
                                                                                as_str(): "u8",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1615,
                                                            end: 1616,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1617,
                                                                    end: 1618,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U32,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1618,
                                                                            end: 1621,
                                                                            as_str(): "u32",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1621,
                                                            end: 1622,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1627,
                                                            end: 1630,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1631,
                                                                end: 1633,
                                                                as_str(): "x4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1633,
                                                                    end: 1634,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1635,
                                                                                end: 1637,
                                                                                as_str(): "u8",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1638,
                                                            end: 1639,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1640,
                                                                    end: 1641,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U64,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1641,
                                                                            end: 1644,
                                                                            as_str(): "u64",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1644,
                                                            end: 1645,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1650,
                                                            end: 1653,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1654,
                                                                end: 1656,
                                                                as_str(): "x5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1656,
                                                                    end: 1657,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1658,
                                                                                end: 1660,
                                                                                as_str(): "u8",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1661,
                                                            end: 1662,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1663,
                                                                    end: 1664,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1664,
                                                            end: 1665,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1671,
                                                            end: 1674,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1675,
                                                                end: 1677,
                                                                as_str(): "y1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1677,
                                                                    end: 1678,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1679,
                                                                                end: 1682,
                                                                                as_str(): "u16",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1683,
                                                            end: 1684,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1685,
                                                                    end: 1686,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U8,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1686,
                                                                            end: 1688,
                                                                            as_str(): "u8",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1688,
                                                            end: 1689,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1694,
                                                            end: 1697,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1698,
                                                                end: 1700,
                                                                as_str(): "y2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1700,
                                                                    end: 1701,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1702,
                                                                                end: 1705,
                                                                                as_str(): "u16",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1706,
                                                            end: 1707,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1708,
                                                                    end: 1709,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U16,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1709,
                                                                            end: 1712,
                                                                            as_str(): "u16",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1712,
                                                            end: 1713,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1718,
                                                            end: 1721,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1722,
                                                                end: 1724,
                                                                as_str(): "y3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1724,
                                                                    end: 1725,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1726,
                                                                                end: 1729,
                                                                                as_str(): "u16",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1730,
                                                            end: 1731,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1732,
                                                                    end: 1733,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U32,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1733,
                                                                            end: 1736,
                                                                            as_str(): "u32",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1736,
                                                            end: 1737,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1742,
                                                            end: 1745,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1746,
                                                                end: 1748,
                                                                as_str(): "y4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1748,
                                                                    end: 1749,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1750,
                                                                                end: 1753,
                                                                                as_str(): "u16",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1754,
                                                            end: 1755,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1756,
                                                                    end: 1757,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U64,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1757,
                                                                            end: 1760,
                                                                            as_str(): "u64",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1760,
                                                            end: 1761,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1766,
                                                            end: 1769,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1770,
                                                                end: 1772,
                                                                as_str(): "y5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1772,
                                                                    end: 1773,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1774,
                                                                                end: 1777,
                                                                                as_str(): "u16",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1778,
                                                            end: 1779,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1780,
                                                                    end: 1781,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1781,
                                                            end: 1782,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1788,
                                                            end: 1791,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1792,
                                                                end: 1794,
                                                                as_str(): "z1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1794,
                                                                    end: 1795,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1796,
                                                                                end: 1799,
                                                                                as_str(): "u32",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1800,
                                                            end: 1801,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1802,
                                                                    end: 1803,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U8,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1803,
                                                                            end: 1805,
                                                                            as_str(): "u8",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1805,
                                                            end: 1806,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1811,
                                                            end: 1814,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1815,
                                                                end: 1817,
                                                                as_str(): "z2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1817,
                                                                    end: 1818,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1819,
                                                                                end: 1822,
                                                                                as_str(): "u32",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1823,
                                                            end: 1824,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1825,
                                                                    end: 1826,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U16,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1826,
                                                                            end: 1829,
                                                                            as_str(): "u16",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1829,
                                                            end: 1830,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1835,
                                                            end: 1838,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1839,
                                                                end: 1841,
                                                                as_str(): "z3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1841,
                                                                    end: 1842,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1843,
                                                                                end: 1846,
                                                                                as_str(): "u32",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1847,
                                                            end: 1848,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1849,
                                                                    end: 1850,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U32,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1850,
                                                                            end: 1853,
                                                                            as_str(): "u32",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1853,
                                                            end: 1854,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1859,
                                                            end: 1862,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1863,
                                                                end: 1865,
                                                                as_str(): "z4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1865,
                                                                    end: 1866,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1867,
                                                                                end: 1870,
                                                                                as_str(): "u32",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1871,
                                                            end: 1872,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1873,
                                                                    end: 1874,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U64,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1874,
                                                                            end: 1877,
                                                                            as_str(): "u64",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1877,
                                                            end: 1878,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1883,
                                                            end: 1886,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1887,
                                                                end: 1889,
                                                                as_str(): "z5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1889,
                                                                    end: 1890,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1891,
                                                                                end: 1894,
                                                                                as_str(): "u32",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1895,
                                                            end: 1896,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1897,
                                                                    end: 1898,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1898,
                                                            end: 1899,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1905,
                                                            end: 1908,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1909,
                                                                end: 1911,
                                                                as_str(): "w1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1911,
                                                                    end: 1912,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1913,
                                                                                end: 1916,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1917,
                                                            end: 1918,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1919,
                                                                    end: 1920,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U8,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1920,
                                                                            end: 1922,
                                                                            as_str(): "u8",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1922,
                                                            end: 1923,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1928,
                                                            end: 1931,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1932,
                                                                end: 1934,
                                                                as_str(): "w2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1934,
                                                                    end: 1935,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1936,
                                                                                end: 1939,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1940,
                                                            end: 1941,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1942,
                                                                    end: 1943,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U16,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1943,
                                                                            end: 1946,
                                                                            as_str(): "u16",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1946,
                                                            end: 1947,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1952,
                                                            end: 1955,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1956,
                                                                end: 1958,
                                                                as_str(): "w3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1958,
                                                                    end: 1959,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1960,
                                                                                end: 1963,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1964,
                                                            end: 1965,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1966,
                                                                    end: 1967,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U32,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1967,
                                                                            end: 1970,
                                                                            as_str(): "u32",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1970,
                                                            end: 1971,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1976,
                                                            end: 1979,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 1980,
                                                                end: 1982,
                                                                as_str(): "w4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1982,
                                                                    end: 1983,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 1984,
                                                                                end: 1987,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1988,
                                                            end: 1989,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 1990,
                                                                    end: 1991,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U64,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 1991,
                                                                            end: 1994,
                                                                            as_str(): "u64",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 1994,
                                                            end: 1995,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2000,
                                                            end: 2003,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2004,
                                                                end: 2006,
                                                                as_str(): "w5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: Some(
                                                        (
                                                            ColonToken {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 2006,
                                                                    end: 2007,
                                                                    as_str(): ":",
                                                                },
                                                            },
                                                            Path(
                                                                PathType {
                                                                    root_opt: None,
                                                                    prefix: PathTypeSegment {
                                                                        name: BaseIdent {
                                                                            name_override_opt: None,
                                                                            span: Span {
                                                                                src (ptr): 0x00007fe0dc169290,
                                                                                path: Some(
                                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                                ),
                                                                                start: 2008,
                                                                                end: 2011,
                                                                                as_str(): "u64",
                                                                            },
                                                                            is_raw_ident: false,
                                                                        },
                                                                        generics_opt: None,
                                                                    },
                                                                    suffix: [],
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2012,
                                                            end: 2013,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 2014,
                                                                    end: 2015,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2015,
                                                            end: 2016,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2022,
                                                            end: 2025,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2026,
                                                                end: 2028,
                                                                as_str(): "v1",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2029,
                                                            end: 2030,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 2031,
                                                                    end: 2032,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U8,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 2032,
                                                                            end: 2034,
                                                                            as_str(): "u8",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2034,
                                                            end: 2035,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2040,
                                                            end: 2043,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2044,
                                                                end: 2046,
                                                                as_str(): "v2",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2047,
                                                            end: 2048,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 2049,
                                                                    end: 2050,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U16,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 2050,
                                                                            end: 2053,
                                                                            as_str(): "u16",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2053,
                                                            end: 2054,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2059,
                                                            end: 2062,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2063,
                                                                end: 2065,
                                                                as_str(): "v3",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2066,
                                                            end: 2067,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 2068,
                                                                    end: 2069,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U32,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 2069,
                                                                            end: 2072,
                                                                            as_str(): "u32",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2072,
                                                            end: 2073,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2078,
                                                            end: 2081,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2082,
                                                                end: 2084,
                                                                as_str(): "v4",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2085,
                                                            end: 2086,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 2087,
                                                                    end: 2088,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: Some(
                                                                    (
                                                                        U64,
                                                                        Span {
                                                                            src (ptr): 0x00007fe0dc169290,
                                                                            path: Some(
                                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                            ),
                                                                            start: 2088,
                                                                            end: 2091,
                                                                            as_str(): "u64",
                                                                        },
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2091,
                                                            end: 2092,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Let(
                                                StatementLet {
                                                    let_token: LetToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2097,
                                                            end: 2100,
                                                            as_str(): "let",
                                                        },
                                                    },
                                                    pattern: Var {
                                                        reference: None,
                                                        mutable: None,
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2101,
                                                                end: 2103,
                                                                as_str(): "v5",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                    },
                                                    ty_opt: None,
                                                    eq_token: EqToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2104,
                                                            end: 2105,
                                                            as_str(): "=",
                                                        },
                                                    },
                                                    expr: Literal(
                                                        Int(
                                                            LitInt {
                                                                span: Span {
                                                                    src (ptr): 0x00007fe0dc169290,
                                                                    path: Some(
                                                                        "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                    ),
                                                                    start: 2106,
                                                                    end: 2107,
                                                                    as_str(): "4",
                                                                },
                                                                parsed: 4,
                                                                ty_opt: None,
                                                            },
                                                        ),
                                                    ),
                                                    semicolon_token: SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2107,
                                                            end: 2108,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                },
                                            ),
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2114,
                                                                        end: 2116,
                                                                        as_str(): "x1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2116,
                                                            end: 2117,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2117,
                                                                end: 2123,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2123,
                                                            end: 2125,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2125,
                                                            end: 2126,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2131,
                                                                        end: 2133,
                                                                        as_str(): "x2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2133,
                                                            end: 2134,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2134,
                                                                end: 2140,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2140,
                                                            end: 2142,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2142,
                                                            end: 2143,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2148,
                                                                        end: 2150,
                                                                        as_str(): "x3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2150,
                                                            end: 2151,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2151,
                                                                end: 2157,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2157,
                                                            end: 2159,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2159,
                                                            end: 2160,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2165,
                                                                        end: 2167,
                                                                        as_str(): "x4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2167,
                                                            end: 2168,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2168,
                                                                end: 2174,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2174,
                                                            end: 2176,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2176,
                                                            end: 2177,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2182,
                                                                        end: 2184,
                                                                        as_str(): "x5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2184,
                                                            end: 2185,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2185,
                                                                end: 2191,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2191,
                                                            end: 2193,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2193,
                                                            end: 2194,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2200,
                                                                        end: 2202,
                                                                        as_str(): "y1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2202,
                                                            end: 2203,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2203,
                                                                end: 2210,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2210,
                                                            end: 2212,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2212,
                                                            end: 2213,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2218,
                                                                        end: 2220,
                                                                        as_str(): "y2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2220,
                                                            end: 2221,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2221,
                                                                end: 2228,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2228,
                                                            end: 2230,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2230,
                                                            end: 2231,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2236,
                                                                        end: 2238,
                                                                        as_str(): "y3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2238,
                                                            end: 2239,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2239,
                                                                end: 2246,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2246,
                                                            end: 2248,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2248,
                                                            end: 2249,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2254,
                                                                        end: 2256,
                                                                        as_str(): "y4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2256,
                                                            end: 2257,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2257,
                                                                end: 2264,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2264,
                                                            end: 2266,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2266,
                                                            end: 2267,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2272,
                                                                        end: 2274,
                                                                        as_str(): "y5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2274,
                                                            end: 2275,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2275,
                                                                end: 2282,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2282,
                                                            end: 2284,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2284,
                                                            end: 2285,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2291,
                                                                        end: 2293,
                                                                        as_str(): "z1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2293,
                                                            end: 2294,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2294,
                                                                end: 2301,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2301,
                                                            end: 2303,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2303,
                                                            end: 2304,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2309,
                                                                        end: 2311,
                                                                        as_str(): "z2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2311,
                                                            end: 2312,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2312,
                                                                end: 2319,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2319,
                                                            end: 2321,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2321,
                                                            end: 2322,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2327,
                                                                        end: 2329,
                                                                        as_str(): "z3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2329,
                                                            end: 2330,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2330,
                                                                end: 2337,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2337,
                                                            end: 2339,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2339,
                                                            end: 2340,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2345,
                                                                        end: 2347,
                                                                        as_str(): "z4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2347,
                                                            end: 2348,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2348,
                                                                end: 2355,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2355,
                                                            end: 2357,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2357,
                                                            end: 2358,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2363,
                                                                        end: 2365,
                                                                        as_str(): "z5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2365,
                                                            end: 2366,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2366,
                                                                end: 2373,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2373,
                                                            end: 2375,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2375,
                                                            end: 2376,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2382,
                                                                        end: 2384,
                                                                        as_str(): "w1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2384,
                                                            end: 2385,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2385,
                                                                end: 2392,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2392,
                                                            end: 2394,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2394,
                                                            end: 2395,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2400,
                                                                        end: 2402,
                                                                        as_str(): "w2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2402,
                                                            end: 2403,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2403,
                                                                end: 2410,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2410,
                                                            end: 2412,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2412,
                                                            end: 2413,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2418,
                                                                        end: 2420,
                                                                        as_str(): "w3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2420,
                                                            end: 2421,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2421,
                                                                end: 2428,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2428,
                                                            end: 2430,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2430,
                                                            end: 2431,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2436,
                                                                        end: 2438,
                                                                        as_str(): "w4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2438,
                                                            end: 2439,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2439,
                                                                end: 2446,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2446,
                                                            end: 2448,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2448,
                                                            end: 2449,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2454,
                                                                        end: 2456,
                                                                        as_str(): "w5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2456,
                                                            end: 2457,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2457,
                                                                end: 2464,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2464,
                                                            end: 2466,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2466,
                                                            end: 2467,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2473,
                                                                        end: 2475,
                                                                        as_str(): "v1",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2475,
                                                            end: 2476,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2476,
                                                                end: 2482,
                                                                as_str(): "foo_u8",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2482,
                                                            end: 2484,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2484,
                                                            end: 2485,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2490,
                                                                        end: 2492,
                                                                        as_str(): "v2",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2492,
                                                            end: 2493,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2493,
                                                                end: 2500,
                                                                as_str(): "foo_u16",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2500,
                                                            end: 2502,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2502,
                                                            end: 2503,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2508,
                                                                        end: 2510,
                                                                        as_str(): "v3",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2510,
                                                            end: 2511,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2511,
                                                                end: 2518,
                                                                as_str(): "foo_u32",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2518,
                                                            end: 2520,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2520,
                                                            end: 2521,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2526,
                                                                        end: 2528,
                                                                        as_str(): "v4",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2528,
                                                            end: 2529,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2529,
                                                                end: 2536,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2536,
                                                            end: 2538,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2538,
                                                            end: 2539,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                            Expr {
                                                expr: MethodCall {
                                                    target: Path(
                                                        PathExpr {
                                                            root_opt: None,
                                                            prefix: PathExprSegment {
                                                                name: BaseIdent {
                                                                    name_override_opt: None,
                                                                    span: Span {
                                                                        src (ptr): 0x00007fe0dc169290,
                                                                        path: Some(
                                                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                        ),
                                                                        start: 2544,
                                                                        end: 2546,
                                                                        as_str(): "v5",
                                                                    },
                                                                    is_raw_ident: false,
                                                                },
                                                                generics_opt: None,
                                                            },
                                                            suffix: [],
                                                            incomplete_suffix: false,
                                                        },
                                                    ),
                                                    dot_token: DotToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2546,
                                                            end: 2547,
                                                            as_str(): ".",
                                                        },
                                                    },
                                                    path_seg: PathExprSegment {
                                                        name: BaseIdent {
                                                            name_override_opt: None,
                                                            span: Span {
                                                                src (ptr): 0x00007fe0dc169290,
                                                                path: Some(
                                                                    "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                                ),
                                                                start: 2547,
                                                                end: 2554,
                                                                as_str(): "foo_u64",
                                                            },
                                                            is_raw_ident: false,
                                                        },
                                                        generics_opt: None,
                                                    },
                                                    contract_args_opt: None,
                                                    args: Parens {
                                                        inner: Punctuated {
                                                            value_separator_pairs: [],
                                                            final_value_opt: None,
                                                        },
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2554,
                                                            end: 2556,
                                                            as_str(): "()",
                                                        },
                                                    },
                                                },
                                                semicolon_token_opt: Some(
                                                    SemicolonToken {
                                                        span: Span {
                                                            src (ptr): 0x00007fe0dc169290,
                                                            path: Some(
                                                                "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                                            ),
                                                            start: 2556,
                                                            end: 2557,
                                                            as_str(): ";",
                                                        },
                                                    },
                                                ),
                                            },
                                        ],
                                        final_expr_opt: None,
                                    },
                                    span: Span {
                                        src (ptr): 0x00007fe0dc169290,
                                        path: Some(
                                            "/tmp/SWAY_LSP_TEMP_DIRCIvLPg/integer_type_inference/src/main.sw",
                                        ),
                                        start: 964,
                                        end: 2559,
                                        as_str(): "{\n    /* Make sure that the resulting types of constants are correct */\n    X1.foo_u8();\n    X2.foo_u8();\n    X3.foo_u8();\n    X4.foo_u8();\n    X5.foo_u8();\n\n    Y1.foo_u16();\n    Y2.foo_u16();\n    Y3.foo_u16();\n    Y4.foo_u16();\n    Y5.foo_u16();\n\n    Z1.foo_u32();\n    Z2.foo_u32();\n    Z3.foo_u32();\n    Z4.foo_u32();\n    Z5.foo_u32();\n\n    W1.foo_u64();\n    W2.foo_u64();\n    W3.foo_u64();\n    W4.foo_u64();\n    W5.foo_u64();\n\n    V1.foo_u8();\n    V2.foo_u16();\n    V3.foo_u32();\n    V4.foo_u64();\n    V5.foo_u64();\n\n    /* Make sure that the resulting types of variables are correct */\n    let x1: u8 = 4u8;\n    let x2: u8 = 4u16;\n    let x3: u8 = 4u32;\n    let x4: u8 = 4u64;\n    let x5: u8 = 4;\n\n    let y1: u16 = 4u8;\n    let y2: u16 = 4u16;\n    let y3: u16 = 4u32;\n    let y4: u16 = 4u64;\n    let y5: u16 = 4;\n\n    let z1: u32 = 4u8;\n    let z2: u32 = 4u16;\n    let z3: u32 = 4u32;\n    let z4: u32 = 4u64;\n    let z5: u32 = 4;\n\n    let w1: u64 = 4u8;\n    let w2: u64 = 4u16;\n    let w3: u64 = 4u32;\n    let w4: u64 = 4u64;\n    let w5: u64 = 4;\n\n    let v1 = 4u8;\n    let v2 = 4u16;\n    let v3 = 4u32;\n    let v4 = 4u64;\n    let v5 = 4;\n\n    x1.foo_u8();\n    x2.foo_u8();\n    x3.foo_u8();\n    x4.foo_u8();\n    x5.foo_u8();\n\n    y1.foo_u16();\n    y2.foo_u16();\n    y3.foo_u16();\n    y4.foo_u16();\n    y5.foo_u16();\n\n    z1.foo_u32();\n    z2.foo_u32();\n    z3.foo_u32();\n    z4.foo_u32();\n    z5.foo_u32();\n\n    w1.foo_u64();\n    w2.foo_u64();\n    w3.foo_u64();\n    w4.foo_u64();\n    w5.foo_u64();\n\n    v1.foo_u8();\n    v2.foo_u16();\n    v3.foo_u32();\n    v4.foo_u64();\n    v5.foo_u64();\n}",
                                    },
                                },
                            },
                        ),
                    },
                ],
            },
            submodules: [],
        },
    },
)
