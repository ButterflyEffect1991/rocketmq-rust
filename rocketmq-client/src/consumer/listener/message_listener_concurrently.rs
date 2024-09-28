/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::sync::Arc;

use rocketmq_common::common::message::message_ext::MessageExt;

use crate::consumer::listener::consume_concurrently_context::ConsumeConcurrentlyContext;
use crate::consumer::listener::consume_concurrently_status::ConsumeConcurrentlyStatus;
use crate::Result;

pub trait MessageListenerConcurrently: Sync + Send {
    fn consume_message(
        &self,
        msgs: &[&MessageExt],
        context: &ConsumeConcurrentlyContext,
    ) -> Result<ConsumeConcurrentlyStatus>;
}

pub type ArcBoxMessageListenerConcurrently = Arc<Box<dyn MessageListenerConcurrently>>;

pub type MessageListenerConcurrentlyFn = Arc<
    dyn Fn(Vec<MessageExt>, ConsumeConcurrentlyContext) -> Result<ConsumeConcurrentlyStatus>
        + Send
        + Sync,
>;