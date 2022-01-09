use std::{cmp, rc::Rc};

use super::builtin_exceptions::BuiltInExceptions;
use crate::message::Message;

pub struct CommandSyntaxException {
    type_: BuiltInExceptions,
    message: Message,
    input: Option<String>,
    cursor: Option<usize>,
}

const CONTEXT_AMOUNT: usize = 10;
const ENABLE_COMMAND_STACK_TRACES: bool = true;

impl CommandSyntaxException {
    pub fn new(type_: BuiltInExceptions, message: Message, input: &str, cursor: usize) -> Self {
        Self {
            type_,
            message,
            input: Some(input.to_string()),
            cursor: Some(cursor),
        }
    }

    pub fn create(type_: BuiltInExceptions, message: Message) -> Self {
        Self {
            type_,
            message,
            input: None,
            cursor: None,
        }
    }

    pub fn message(&self) -> String {
        let mut message = self.message.string();
        let context = self.context();
        if let Some(context) = context {
            message.push_str(&format!(
                " at position {}: {}",
                self.cursor.unwrap_or(usize::MAX),
                context
            ));
        }
        message
    }

    pub fn raw_message(&self) -> &Message {
        &self.message
    }

    pub fn context(&self) -> Option<String> {
        if let Some(input) = &self.input {
            if let Some(cursor) = self.cursor {
                let mut builder = String::new();
                let cursor = cmp::min(input.len(), cursor);

                if cursor > CONTEXT_AMOUNT {
                    builder.push_str("...");
                }

                builder.push_str(&input[cmp::max(0, cursor - CONTEXT_AMOUNT)..cursor]);
                builder.push_str("<--[HERE]");

                return Some(builder);
            }
        }
        None
    }

    pub fn get_type(&self) -> &BuiltInExceptions {
        &self.type_
    }

    pub fn input(&self) -> String {
        self.input()
    }

    pub fn cursor(&self) -> Option<usize> {
        self.cursor
    }
}
