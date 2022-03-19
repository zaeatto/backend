use revolt_quark::{models::User, perms, Db, EmptyResponse, Permission, Ref, Result};

/// # Acknowledge Message
///
/// Lets the server and all other clients know that we've seen this message id in this channel.
#[openapi(tag = "Messaging")]
#[put("/<target>/ack/<message>")]
pub async fn req(db: &Db, user: User, target: Ref, message: Ref) -> Result<EmptyResponse> {
    let channel = target.as_channel(db).await?;
    perms(&user)
        .channel(&channel)
        .throw_permission(db, Permission::ViewChannel)
        .await?;

    channel
        .ack(db, &user.id, &message.id)
        .await
        .map(|_| EmptyResponse)
}
