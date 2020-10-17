## At the moment it's just an idea

example
```rust
use http_types_ext::ValidDeserialize;
use tide::prelude::*;
use tide::Request;
use validator_derive::Validate;

#[derive(Deserialize, Validate)]
struct LoginInfo {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/login")
        .post(|mut request: Request<()>| async move {
            let _login_info: LoginInfo = request.valid_json().await?; // this will raise the errors
            Ok("Wellcome")
        });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

In order to use the helper error mapper
```rust
use http_types_ext::validation::map_validation_errors;
use http_types_ext::ValidDeserialize;
use tide::prelude::*;
use tide::Request;
use validator_derive::Validate;

#[derive(Deserialize, Validate)]
struct LoginInfo {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.with(tide::utils::After(map_validation_errors))
        .at("/login")
        .post(|mut request: Request<()>| async move {
            let _login_info: LoginInfo = request.valid_json().await?; // this will raise the errors
            Ok("Wellcome")
        });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

### This is not a published crate
