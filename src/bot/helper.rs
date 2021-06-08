use frankenstein::Message;

pub struct MessageUnpacker<'a> {
    message: &'a Message
}

impl<'a> MessageUnpacker<'a> {
    pub fn new(message: &'a Message) -> Self {
        Self {
            message
        }
    }

    pub fn unpack(&self) -> Option<(isize, String)> {
        if let Some(user) = &self.message.from {
            if let Some(text) = &self.message.text {
                return Some((user.id, text.to_string()));
            } else {
                log::error!("Failed to unpack text in message!")
            }
        } else {
            log::error!("Failed to find user id in message!");
        }

        None
    }
}
