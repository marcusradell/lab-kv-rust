pub async fn create_identity(email: &str) -> Result<String, String> {
    if email.is_empty() {
        return Err("Email must be nonempty.".to_string());
    }

    Ok("##jwt##".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_identity_must_have_nonempty_email() {
        let email = "";

        let actual = create_identity(email).await.unwrap_err();

        assert_eq!(actual, "Email must be nonempty.".to_string())
    }

    #[tokio::test]
    async fn create_identity_returns_jwt() {
        let actual = create_identity("me@example.com").await.unwrap();

        assert_eq!(actual, "##jwt##".to_string())
    }
}
