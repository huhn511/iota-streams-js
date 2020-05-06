use iota_streams::app_channels::api::tangle::{Author, Transport};
use failure::Fallible;

pub fn start_a_new_channel<T: Transport>(author: &mut Author, client: &mut T, send_opt: T::SendOptions) -> Fallible<()> {
    let announcement = author.announce()?;

    println!("Message identifier: {}", announcement.link.msgid);
    client.send_message_with_options(&announcement, send_opt)?;
    println!("Channel published");
    //OK(())
    Ok(())
}
