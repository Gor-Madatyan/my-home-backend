#[macro_export]
macro_rules! serialize_into_request {
    ($($name:ident),*) => {
        $(
        impl IntoResponse for $name {
            fn into_response(self) -> Response {
                serde_json::to_string(&self)
                    .map_err(|e| AppError::from(e))
                    .into_response()
        }
        }
        )*
    };
}

#[macro_export]
macro_rules! sanitize {
    ($($name:ident),*) => {
        $(
         let $name = format!("\"{}\"", $name.replace("\"", "\"\""));
        )*
    }
}
