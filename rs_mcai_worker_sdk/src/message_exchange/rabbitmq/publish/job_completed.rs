use lapin::message::Delivery;
use lapin::options::{BasicAckOptions, BasicPublishOptions, BasicRejectOptions};
use lapin::{BasicProperties, Channel, Promise};

use crate::{
  message_exchange::rabbitmq::{EXCHANGE_NAME_JOB_RESPONSE, ROUTING_KEY_JOB_COMPLETED},
  JobResult,
};
use std::sync::Arc;

pub fn job_completed(
  channel: Arc<Channel>,
  delivery: &Delivery,
  job_result: &JobResult,
) -> Promise<()> {
  let msg = json!(job_result).to_string();

  let result = channel
    .basic_publish(
      EXCHANGE_NAME_JOB_RESPONSE,
      ROUTING_KEY_JOB_COMPLETED,
      BasicPublishOptions::default(),
      msg.as_bytes().to_vec(),
      BasicProperties::default(),
    )
    .wait()
    .is_ok();

  if result {
    channel.basic_ack(
      delivery.delivery_tag,
      BasicAckOptions::default(), /*not requeue*/
    )
  } else {
    channel.basic_reject(
      delivery.delivery_tag,
      BasicRejectOptions { requeue: true }, /*requeue*/
    )
  }
}