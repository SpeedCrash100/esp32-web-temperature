//!
//! ector-based I2C bus
//!

use defmt::info;
use ector::{Actor, ActorRequest};
use embedded_hal_async::i2c::{Error, ErrorKind};

pub struct I2cActor<I2C> {
    pub i2c: I2C,
}

pub struct Request {
    address: u8,
    operations: &'static mut [embedded_hal_async::i2c::Operation<'static>],
}

pub type Response = Result<(), ErrorKind>;

impl<I2C> Actor for I2cActor<I2C>
where
    I2C: embedded_hal_async::i2c::I2c,
{
    type Message = ector::Request<Request, Response>;

    async fn on_mount<M>(&mut self, _: ector::DynamicAddress<Self::Message>, mut inbox: M) -> !
    where
        M: ector::Inbox<Self::Message>,
    {
        info!("i2c actor started");

        loop {
            let mut input_request = inbox.next().await;
            let request = input_request.get_mut();

            match self
                .i2c
                .transaction(request.address, request.operations)
                .await
            {
                Ok(_) => input_request.reply(Ok(())).await,
                Err(err) => input_request.reply(Err(err.kind())).await,
            };
        }
    }
}

/// # Warning
///
/// Do not use in Select
/// This implementation is NOT cancel-safe. It will panic it you try to do so
///
///
#[derive(Clone)]
pub struct EctorI2c {
    pub address: ector::DynamicAddress<ector::Request<Request, Response>>,
}

impl embedded_hal_async::i2c::ErrorType for EctorI2c {
    type Error = ErrorKind;
}

impl embedded_hal_async::i2c::I2c for EctorI2c {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        // Safety:
        // - the request completion ensures buffer will not used anymore by actor
        // - because cancel is forbidden(protected with panic), even if user tries we will stop using buffer y stopping program
        //
        let static_operations: &'static mut [embedded_hal_async::i2c::Operation<'static>] =
            unsafe { core::mem::transmute(operations) };

        self.address
            .request(Request {
                address,
                operations: static_operations,
            })
            .await
    }
}
