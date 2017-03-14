use errors::*;
use anterofit::{Adapter, Url, JsonAdapter};
use anterofit::net::request::RequestHead;
use anterofit::net::intercept::Interceptor;
use anterofit::net::header::{Headers, Basic, Accept, qitem, Authorization, UserAgent};
use services::{User, UserProfile, Feeds, Thread};
use services::chrono::{DateTime, UTC};
use services::mime::{Mime, TopLevel, SubLevel};

#[derive(Debug)]
pub struct Root {
    adapter: JsonAdapter,
}

pub struct RootInterceptor {
    username: String,
    password: String,
}

impl Interceptor for RootInterceptor {
    fn intercept(&self, req: &mut RequestHead) {
        let mut headers = Headers::new();
        // Authorization: Basic
        headers.set(Authorization(Basic {
            username: self.username.clone(),
            password: Some(self.password.clone()),
        }));

        // Accept: application/vnd.github.v3+json
        headers.set(Accept(vec![qitem(Mime(TopLevel::Application,
                                           SubLevel::Ext("vnd.github.v3.raw+json".to_owned()),
                                           vec![]))]));

        // FIXME: come up with something
        headers.set(UserAgent("gh-rs-cli/0.0.1".to_owned()));
        req.headers(&headers);
    }
}

impl Root {
    pub fn new(url: &str, username: &str, password: &str) -> Result<Self> {
        let interceptor = RootInterceptor {
            username: username.to_string(),
            password: password.to_string(),
        };
        Ok(Root {
            adapter: Adapter::builder()
                .serialize_json()
                .base_url(Url::parse(url)?)
                .interceptor(interceptor)
                .build(),
        })
    }
}

service! {
    pub trait RootService {
        fn get_authenticated_user(&self) -> User {
            GET("user")
        }

        fn get_user(&self, name: &str) -> User {
            GET("users/{}", name)
        }

        fn get_users(&self) -> Vec<UserProfile> {
            GET("users")
        }

        fn get_feeds(&self) ->  Feeds {
            GET("feeds")
        }

        fn get_notifications(&self, opts: Option<GetNotificationsParams>) -> Vec<Thread> {
            GET("notifications");
            |mut builder| {
                if let Some(ref opts) = opts {
                    if let Some(all) = opts.all {
                        builder.head_mut().query(&[("all", &all)]);
                    }
                    if let Some(participating) = opts.participating {
                        builder.head_mut().query(&[("participating", &participating)]);
                    }
                    if let Some(since) = opts.since {
                        builder.head_mut().query(&[("since", since.to_rfc3339())]);
                    }
                    if let Some(before) = opts.before {
                        builder.head_mut().query(&[("before", before.to_rfc3339())]);
                    }
                    if let Some(page) = opts.page {
                        builder.head_mut().query(&[("page", &page)]);
                    }
                    if let Some(per_page) = opts.per_page {
                        builder.head_mut().query(&[("per_page", &per_page)]);
                    }
                }
                Ok(builder)
            }
        }
    }

    // fn get_user_repos(&self, user: &str, opts: Option<GetUserReposParams>) -> Vec<

    impl for Root {
        |this| &this.adapter
    }

    impl[T] for T [where T: AsRef<Root>] {
        |this| &this.as_ref().adapter
    }
}

pub struct GetNotificationsParams {
    all: Option<bool>,
    participating: Option<bool>,
    since: Option<DateTime<UTC>>,
    before: Option<DateTime<UTC>>,
    page: Option<u32>,
    per_page: Option<u32>,
}
