pub struct SubscriptionToken(String);

impl SubscriptionToken {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.chars().any(|c| !c.is_alphanumeric()) {
            return Err("Subscription token is alphanumeric.".to_string());
        }
        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriptionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
