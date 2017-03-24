use serenity::client::{Context, CACHE};
use serenity::model::*;
use serenity::ext::cache::*;
use serenity::utils::MessageBuilder;
use serenity::utils::builder::*;
use std::ops::Deref;

pub fn exec_cmd(_context: &mut Context, message: &Message, args: Vec<String>) -> Result<(), String> {

    if message.is_private() {
        message.channel_id.send_message(|m| m.content("This command cannot be used in private messages."));
        return Ok(())
    }

    let mut embeded_msg = CreateEmbed::default();
    let mut member_to_parse = message.clone().author;

    /*if args.len() >= 1 {
        match args. {
            "server" => {
                
            },
            _ => if let Some(author) = search_for_user(args[0]) {
                    member_to_parse = author;
                    } else {
                        return Err("User not found".to_string())
                    }
        }
    }*/

    //embeded_msg = embeded_msg.

    Ok(())
}

#[inline]
fn build_member_embed(member: Member) -> CreateEmbed {

    let user = member.user
                .read().unwrap()
                .deref()
                .clone();

    CreateEmbed::default()
        .author(|a| a
            .name(user.name.as_str())
            .icon_url(user.default_avatar_url().unwrap().as_str()))
        .footer(|f| f.value(MessageBuilder::new()
            .push("Member since ")
            .push(member.joined_at.as_str())))
        .field(|f| f
            .name("ID")
            .value(user.UserId.to_string().as_str()))
        .field(|f| f
            .name("Created")
            .value(user.created_at().to_string().as_str()))
        .field(|f| f
            .name("Roles")
            .value(MessageBuilder::new()
                .push()))
}

fn search_for_user(name: String) -> Option<Member> {
    let cache = CACHE.read().unwrap();

    let member_result = cache.deref()
                             .guilds
                             .iter()
                             .find(|&(gid, _arc)| *gid == 273534239310479360)
                             .unwrap()
                             .1
                             .read()
                             .unwrap()
                             .members
                             .iter()
                             .find(|&(_, member)| member.user.read().unwrap().name == name.trim_left_matches('@').to_string());

    let result = match member_result {
        Some((ref _uid, ref mem)) => {
            Some(*mem.clone())
        },
        None => None
    };

    result
}