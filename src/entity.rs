use std::borrow::Borrow;
use errors::*;
use client::{Client, AsUrlParams, Path};
use serde::{Serialize, Deserialize};
use reqwest::Response;

pub trait Entity<'a>: Serialize + Deserialize + Default {
    fn set_client(&mut self, client: &'a Client);
    fn get_client(&self) -> Option<&'a Client>;

    fn get_entity<E, P, Q>(&'a self, path: P, params: Option<Q>) -> Result<(Response, E)>
        where E: Entity<'a>,
              P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
              Q: AsUrlParams,
              <<Q as AsUrlParams>::I as IntoIterator>::Item: Borrow<(Q::K, Q::V)>
    {
        if let Some(client) = self.get_client() {
            let mut entity: E = Default::default();
            let resp = client.get_entity(path, params, &mut entity)?;
            Ok((resp, entity))
        } else {
            Err(ErrorKind::UninitializedEntity.into())
        }
    }

    fn get_entities<E, P, Q>(&'a self, path: P, params: Option<Q>) -> Result<(Response, Vec<E>)>
        where E: Entity<'a>,
              P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
              Q: AsUrlParams,
              <<Q as AsUrlParams>::I as IntoIterator>::Item: Borrow<(Q::K, Q::V)>
    {
        if let Some(client) = self.get_client() {
            let mut entities = Vec::<E>::new();
            let resp = client.get_entities(path, params, &mut entities)?;
            Ok((resp, entities))
        } else {
            Err(ErrorKind::UninitializedEntity.into())
        }
    }

    fn put<S, P>(&'a self, path: P, payload: &S) -> Result<Response>
        where S: Serialize,
              P: Path, <P as IntoIterator>::Item: Borrow<P::V>,
    {
        if let Some(client) = self.get_client() {
            Ok(client.put(path, payload)?)
        } else {
            Err(ErrorKind::UninitializedEntity.into())
        }
    }

    fn patch<S, P>(&'a self, path: P, payload: &S) -> Result<Response>
        where S: Serialize,
              P: Path,
              <P as IntoIterator>::Item: Borrow<P::V>,
    {
        if let Some(client) = self.get_client() {
            Ok(client.patch(path, payload)?)
        } else {
            Err(ErrorKind::UninitializedEntity.into())
        }
    }
}
