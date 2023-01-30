
TyStructDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe062436520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
            ),
            start: 16,
            end: 17,
            as_str(): "A",
        },
        is_raw_ident: false,
    },
    fields: [
        TyStructField {
            name: BaseIdent {
                name_override_opt: None,
                span: Span {
                    src (ptr): 0x00007fe062436520,
                    path: Some(
                        "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                    ),
                    start: 24,
                    end: 28,
                    as_str(): "addr",
                },
                is_raw_ident: false,
            },
            type_id: TypeId(
                9112,
            ),
            initial_type_id: TypeId(
                31630,
            ),
            span: Span {
                src (ptr): 0x00007fe062436520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                ),
                start: 24,
                end: 37,
                as_str(): "addr: Address",
            },
            type_span: Span {
                src (ptr): 0x00007fe062436520,
                path: Some(
                    "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
                ),
                start: 30,
                end: 37,
                as_str(): "Address",
            },
            attributes: {},
        },
    ],
    type_parameters: [],
    visibility: Private,
    span: Span {
        src (ptr): 0x00007fe062436520,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
        ),
        start: 9,
        end: 40,
        as_str(): "struct A {\n    addr: Address,\n}",
    },
    attributes: {},
}
TyFunctionDeclaration {
    name: BaseIdent {
        name_override_opt: None,
        span: Span {
            src (ptr): 0x00007fe062436520,
            path: Some(
                "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
            ),
            start: 105,
            end: 109,
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
        src (ptr): 0x00007fe062436520,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
        ),
        start: 102,
        end: 115,
        as_str(): "fn main() {\n}",
    },
    attributes: {},
    return_type: TypeId(
        31633,
    ),
    initial_return_type: TypeId(
        31632,
    ),
    type_parameters: [],
    return_type_span: Span {
        src (ptr): 0x00007fe062436520,
        path: Some(
            "/tmp/SWAY_LSP_TEMP_DIRyFb7oZ/prelude_access/src/main.sw",
        ),
        start: 102,
        end: 111,
        as_str(): "fn main()",
    },
    visibility: Private,
    is_contract_call: false,
    purity: Pure,
}

