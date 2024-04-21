use std::{marker::PhantomData, ops::Deref};

use crate::{Event, EventKind, HandleError};

pub struct Config<T>(T);

impl<T> Deref for Config<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Utxo<D = ()> {
    pub datum: D,
}

pub trait PlutusData {}

pub struct UtxoRouter<F, D, C>
where
    F: FnOnce(Utxo<D>, Config<C>) -> Result<(), HandleError>,
{
    handler: Option<F>,
    _d: PhantomData<D>,
    _c: PhantomData<C>,
}

impl<F, D, C> Default for UtxoRouter<F, D, C>
where
    F: FnOnce(Utxo<D>, Config<C>) -> Result<(), HandleError>,
{
    fn default() -> Self {
        Self {
            handler: Default::default(),
            _d: Default::default(),
            _c: Default::default(),
        }
    }
}

impl<F, D, C> UtxoRouter<F, D, C>
where
    F: FnOnce(Utxo<D>, Config<C>) -> Result<(), HandleError>,
{
    pub fn holding_token(self, bech32: &str) -> Self {
        self
    }

    pub fn at_address(self, bech32: &str) -> Self {
        self
    }

    pub fn handle_with(self, func: F) -> Self {
        Self {
            handler: Some(func),
            ..self
        }
    }

    pub fn bind(self, evt: Event) -> Result<(), u32> {
        let utxo = Utxo::from_event(evt.body);
        let config = Config::from_event(evt.config);

        if let Some(f) = self.handler {
            f(utxo, config)
        } else {
            Ok(())
        }
    }
}

pub struct Router;

impl Router {
    pub fn on_utxo<F, D, C>() -> UtxoRouter<F, D, C>
    where
        F: FnOnce(Utxo<D>, Config<C>) -> Result<(), HandleError>,
    {
        UtxoRouter::default()
    }

    pub fn on_timer() -> Self {
        todo!()
    }

    pub fn on_request() -> Self {
        todo!()
    }

    pub fn on_message() -> Self {
        todo!()
    }
}
