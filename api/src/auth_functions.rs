#[server]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let auth = auth::get_session().await?;
    Ok(auth.current_user.unwrap().username.to_string())
}

#[server]
pub async fn login() -> Result<(), ServerFnError> {
    let auth = auth::get_session().await?;
    auth.login_user(2);
    Ok(())
}

#[server]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    let method: axum::http::Method = extract().await?;
    let auth = auth::get_session().await?;
    let current_user = auth.current_user.clone().unwrap_or_default();

    // lets check permissions only and not worry about if they are anon or not
    if !axum_session_auth::Auth::<crate::auth::User, i64, sqlx::SqlitePool>::build(
        [axum::http::Method::POST],
        false,
    )
    .requires(axum_session_auth::Rights::any([
        axum_session_auth::Rights::permission("Category::View"),
        axum_session_auth::Rights::permission("Admin::View"),
    ]))
    .validate(&current_user, &method, None)
    .await
    {
        return Ok(format!(
            "User {}, Does not have permissions needed to view this page please login",
            current_user.username
        ));
    }

    Ok(format!(
        "User has Permissions needed. Here are the Users permissions: {:?}",
        current_user.permissions
    ))
}