use reqwest;
use reqwest::header::{Headers, Basic, Accept, qitem, Authorization};
use mime::{Mime, TopLevel, SubLevel};
use reqwest::{Response, Result, Url};
use entity::Entity;
use std::borrow::Borrow;
use serde::{Serialize};

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    username: String,
    password: String,
    url: String,
}

impl<'a> Client {
    pub fn new(username: String, password: String, url: String) -> Result<Self> {
        Ok(Client {
            client: reqwest::Client::new()?,
            username: username,
            password: password,
            url: url,
        })
    }

    fn get_headers(&self) -> Headers {
        let mut headers = Headers::new();

        // Authorization: Basic QWxhZGRpbjpPcGVuU2VzYW1l
        headers.set(Authorization(Basic {
            username: self.username.clone(),
            password: Some(self.password.clone()),
        }));

        // Accept: application/vnd.github.v3+json
        headers.set(Accept(vec![qitem(Mime(TopLevel::Application,
                                           SubLevel::Ext("vnd.github.v3.raw+json".to_owned()),
                                           vec![]))]));

        headers
    }

    /// Given a path, and url parameters, builds a valid URL from the client's root URL.
    fn get_url<P, Q>(&self, path: P, params: Option<Q>) -> Result<String>
        where P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
              Q: AsUrlParams,
              <<Q as AsUrlParams>::I as IntoIterator>::Item: Borrow<(Q::K, Q::V)>
    {
        let mut url = Url::parse(&self.url)?;
        for part in path {
            url = url.join(part.borrow().as_ref())?;
        }
        let url = if let Some(params) = params.map(|p| p.as_url_params()) {
            Url::parse_with_params(url.as_str(), params)?
        } else {
            let params = EmptyUrlParams {};
            Url::parse_with_params(url.as_str(), params.as_url_params())?
        };
        Ok(url.to_string())
    }

    /// Build a URL from the client's root URL and the given URL path and parameters, then perform
    /// a `GET` request.
    fn get<P, Q>(&self, path: P, params: Option<Q>) -> Result<Response>
        where P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
              Q: AsUrlParams,
              <<Q as AsUrlParams>::I as IntoIterator>::Item: Borrow<(Q::K, Q::V)>
    {
        let url = self.get_url(path, params)?;
        self.client
            .get(url.as_str())
            .headers(self.get_headers())
            .send()
    }

    pub fn get_entity<E, P, Q>(&'a self,
                               path: P,
                               params: Option<Q>,
                               entity: &mut E)
                               -> Result<Response>
        where E: Entity<'a>,
              P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
              Q: AsUrlParams,
              <<Q as AsUrlParams>::I as IntoIterator>::Item: Borrow<(Q::K, Q::V)>
    {
        let mut resp = self.get(path, params)?;
        *entity = resp.json()?;
        entity.set_client(self);
        Ok(resp)
    }

    pub fn get_entities<E, P, Q>(&'a self,
                                 path: P,
                                 params: Option<Q>,
                                 entities: &mut Vec<E>)
                                 -> Result<Response>
        where E: Entity<'a>,
              P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
              Q: AsUrlParams,
              <<Q as AsUrlParams>::I as IntoIterator>::Item: Borrow<(Q::K, Q::V)>
    {
        let mut resp = self.get(path, params)?;
        // let mut s: String = String::new();
        // resp.read_to_string(&mut s);
        // println!("{}", s);
        // *entities = serde_json::from_str(&s)?;
        *entities = resp.json()?;
        for entity in entities {
            entity.set_client(self);
        }
        Ok(resp)
    }

    pub fn put<S, P>(&'a self, path: P, payload: &S) -> Result<Response>
        where S: Serialize,
              P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
    {
        self.client
            .put(self.get_url(path, Some(&EmptyUrlParams{}))?.as_str())
            .headers(self.get_headers())
            .json(payload)
            .send()
    }

    pub fn patch<S, P>(&'a self, path: P, payload: &S) -> Result<Response>
        where S: Serialize,
              P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
    {
        self.client
            .patch(self.get_url(path, Some(&EmptyUrlParams{}))?.as_str())
            .headers(self.get_headers())
            .json(payload)
            .send()
    }
}

pub trait Path: IntoIterator
    where <Self as IntoIterator>::Item: Borrow<Self::V>
{
    type V: AsRef<str>;
}

impl<'a, T: AsRef<str>> Path for &'a Vec<T> {
    type V = T;
}

impl<'a, T: AsRef<str>> Path for &'a mut Vec<T> {
    type V = T;
}

impl<T: AsRef<str>> Path for Vec<T> {
    type V = T;
}

impl<'a, T: AsRef<str>> Path for &'a [T] {
    type V = T;
}

impl<'a, T: AsRef<str>> Path for &'a mut [T] {
    type V = T;
}

macro_rules! impl_path {
    ($n:expr) => (
        impl<'a, T: AsRef<str>> Path for &'a [T; $n] {
            type V = T;
        }

        impl<'a, T: AsRef<str>> Path for &'a mut [T; $n] {
            type V = T;
        }
    );
    ($n:expr, $( $r:expr ),*) => (
        impl_path!($n);
        impl_path!($( $r ),*);
    )
}

impl_path!(0,
           1,
           2,
           3,
           4,
           5,
           6,
           7,
           8,
           9,
           10,
           11,
           12,
           13,
           14,
           15,
           16,
           17,
           18,
           19,
           20,
           21,
           22,
           23,
           24,
           25,
           26,
           27,
           28,
           29,
           30,
           31);

pub trait AsUrlParams
    where <Self::I as IntoIterator>::Item: Borrow<(Self::K, Self::V)>
{
    type I: IntoIterator;
    type K: AsRef<str>;
    type V: AsRef<str>;

    fn as_url_params(self) -> Self::I;
}

impl<'a, T, U> AsUrlParams for &'a Vec<(T, U)>
    where T: AsRef<str>,
          U: AsRef<str>
{
    type I = Self;
    type K = T;
    type V = U;

    fn as_url_params(self) -> Self::I {
        self
    }
}

impl<'a, T, U> AsUrlParams for &'a mut Vec<(T, U)>
    where T: AsRef<str>,
          U: AsRef<str>
{
    type I = Self;
    type K = T;
    type V = U;

    fn as_url_params(self) -> Self::I {
        self
    }
}

impl<T, U> AsUrlParams for Vec<(T, U)>
    where T: AsRef<str>,
          U: AsRef<str>
{
    type I = Self;
    type K = T;
    type V = U;

    fn as_url_params(self) -> Self::I {
        self
    }
}

macro_rules! impl_as_url_params {
    ($n:expr) => (
        impl<'a, T, U> AsUrlParams for &'a [(T, U); $n]
        where T: AsRef<str>,
              U: AsRef<str>,
        {
            type I = Self;
            type K = T;
            type V = U;

            fn as_url_params(self) -> Self::I {
                self
            }
        }

        impl<'a, T, U> AsUrlParams for &'a mut [(T, U); $n]
        where T: AsRef<str>,
              U: AsRef<str>,
        {
            type I = Self;
            type K = T;
            type V = U;

            fn as_url_params(self) -> Self::I {
                self
            }
        }

    );
    ($n:expr, $( $r:expr ),*) => (
        impl_as_url_params!($n);
        impl_as_url_params!($( $r ),*);
    )
}

impl_as_url_params!(0,
                    1,
                    2,
                    3,
                    4,
                    5,
                    6,
                    7,
                    8,
                    9,
                    10,
                    11,
                    12,
                    13,
                    14,
                    15,
                    16,
                    17,
                    18,
                    19,
                    20,
                    21,
                    22,
                    23,
                    24,
                    25,
                    26,
                    27,
                    28,
                    29,
                    30,
                    31);


pub struct EmptyUrlParams;

impl<'a> AsUrlParams for &'a EmptyUrlParams {
    type I = Vec<(&'static str, &'static str)>;
    type K = &'static str;
    type V = &'static str;
    fn as_url_params(self) -> Self::I {
        vec![]
    }
}
