use reqwest;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
    }

    errors {
        UninitializedEntity {
            description("A rest call has been attempted from an entity that was not initialized")
            display("A rest call has been attempted from an entity that was not initialized")
        }
    }
}
