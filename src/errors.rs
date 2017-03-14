use anterofit;
error_chain! {
    foreign_links {
        Anterofit(anterofit::error::Error);
        Url(anterofit::error::UrlError);
    }
}
