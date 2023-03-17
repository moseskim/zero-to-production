use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    // `String`을 더이상 사용하지 않는다!
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
