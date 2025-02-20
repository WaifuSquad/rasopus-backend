#[macro_export]
macro_rules! impl_okapi_json_responder {
    ($type:ty, {
        $(
            $status:literal => {
                description: $desc:expr,
                $(example: $example:expr,)?
            }
        ),* $(,)?
    }) => {
        impl rocket_okapi::response::OpenApiResponderInner for $type {
            fn responses(generator: &mut rocket_okapi::r#gen::OpenApiGenerator) -> rocket_okapi::Result<okapi::openapi3::Responses> {
                Ok(okapi::openapi3::Responses {
                    responses: okapi::map! {
                        $(
                            $status.to_string() => okapi::openapi3::RefOr::Object(rocket_okapi::okapi::openapi3::Response {
                                description: $desc.to_string(),
                                content: okapi::map! {
                                    "application/json".to_string() => {
                                        let media_type = okapi::openapi3::MediaType {
                                            schema: Some(generator.json_schema::<$type>()),$(
                                                example: Some($example),
                                            )?
                                            ..Default::default()
                                        };
                                        media_type
                                    }
                                },
                                ..Default::default()
                            })
                        ),*
                    },
                    ..Default::default()
                })
            }
        }
    };
}
