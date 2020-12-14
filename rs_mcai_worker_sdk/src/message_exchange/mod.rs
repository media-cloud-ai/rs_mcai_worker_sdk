mod local;
mod rabbitmq;

use crate::job::JobProgression;
use crate::{job::Job, JobResult, MessageError, Result};
use async_std::channel::Receiver;
pub use local::LocalExchange;
pub use rabbitmq::RabbitmqExchange;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub enum ResponseMessage {
  Initialized,
  Completed(JobResult),
  Feedback(Feedback),
  Error(MessageError),
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrderMessage {
  InitProcess(Job),
  StartProcess(Job),
  StopProcess(Job),
  StopWorker,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Feedback {
  Progression(JobProgression),
}

pub type SharedExternalExchange = Arc<Mutex<dyn ExternalExchange + Send>>;
pub type SharedInternalExchange = Arc<Mutex<dyn InternalExchange + Send>>;

pub trait ExternalExchange {
  fn send_order(&mut self, message: OrderMessage) -> Result<()>;
  fn next_response(&mut self) -> Result<Option<ResponseMessage>>;
}

pub trait InternalExchange {
  fn send_response(&mut self, message: ResponseMessage) -> Result<()>;
  fn get_response_sender(&self) -> Arc<Mutex<dyn ResponseSender + Send>>;
  fn get_order_receiver(&self) -> Arc<Mutex<Receiver<OrderMessage>>>;
}

pub trait ResponseSender {
  fn send_response(&'_ self, message: ResponseMessage) -> Result<()>;
}