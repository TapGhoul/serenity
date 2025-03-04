use serde::Serialize;

#[cfg(feature = "model")]
use crate::builder::{
    CreateInteractionResponse,
    CreateInteractionResponseFollowup,
    CreateInteractionResponseMessage,
    EditInteractionResponse,
};
#[cfg(feature = "model")]
use crate::http::Http;
use crate::internal::prelude::*;
use crate::model::prelude::*;

/// An interaction triggered by a modal submit.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object).
#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "Self")]
#[non_exhaustive]
pub struct ModalInteraction {
    /// Id of the interaction.
    pub id: InteractionId,
    /// Id of the application this interaction is for.
    pub application_id: ApplicationId,
    /// The data of the interaction which was triggered.
    pub data: ModalInteractionData,
    /// The guild Id this interaction was sent from, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    /// Channel that the interaction was sent from.
    pub channel: Option<PartialChannel>,
    /// The channel Id this interaction was sent from.
    pub channel_id: ChannelId,
    /// The `member` data for the invoking user.
    ///
    /// **Note**: It is only present if the interaction is triggered in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Member>,
    /// The `user` object for the invoking user.
    #[serde(default)]
    pub user: User,
    /// A continuation token for responding to the interaction.
    pub token: FixedString,
    /// Always `1`.
    pub version: u8,
    /// The message this interaction was triggered by
    ///
    /// **Note**: Does not exist if the modal interaction originates from an application command
    /// interaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
    /// Permissions the app or bot has within the channel the interaction was sent from.
    pub app_permissions: Option<Permissions>,
    /// The selected language of the invoking user.
    pub locale: FixedString,
    /// The guild's preferred locale.
    pub guild_locale: Option<FixedString>,
    /// For monetized applications, any entitlements of the invoking user.
    pub entitlements: Vec<Entitlement>,
}

#[cfg(feature = "model")]
impl ModalInteraction {
    /// Gets the interaction response.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if there is no interaction response.
    pub async fn get_response(&self, http: &Http) -> Result<Message> {
        http.get_original_interaction_response(&self.token).await
    }

    /// Creates a response to the interaction received.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the message content is too long. May also return an
    /// [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is an error in
    /// deserializing the API response.
    pub async fn create_response(
        &self,
        http: &Http,
        builder: CreateInteractionResponse<'_>,
    ) -> Result<()> {
        builder.execute(http, self.id, &self.token).await
    }

    /// Edits the initial interaction response.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Model`] if the message content is too long. May also return an
    /// [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is an error in
    /// deserializing the API response.
    pub async fn edit_response(
        &self,
        http: &Http,
        builder: EditInteractionResponse<'_>,
    ) -> Result<Message> {
        builder.execute(http, &self.token).await
    }

    /// Deletes the initial interaction response.
    ///
    /// Does not work on ephemeral messages.
    ///
    /// # Errors
    ///
    /// May return [`Error::Http`] if the API returns an error. Such as if the response was already
    /// deleted.
    pub async fn delete_response(&self, http: &Http) -> Result<()> {
        http.delete_original_interaction_response(&self.token).await
    }

    /// Creates a followup response to the response sent.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Model`] if the content is too long. May also return [`Error::Http`] if the
    /// API returns an error, or [`Error::Json`] if there is an error in deserializing the
    /// response.
    pub async fn create_followup(
        &self,
        http: &Http,
        builder: CreateInteractionResponseFollowup<'_>,
    ) -> Result<Message> {
        builder.execute(http, None, &self.token).await
    }

    /// Edits a followup response to the response sent.
    ///
    /// **Note**: Message contents must be under 2000 unicode code points.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Model`] if the content is too long. May also return [`Error::Http`] if the
    /// API returns an error, or [`Error::Json`] if there is an error in deserializing the
    /// response.
    pub async fn edit_followup(
        &self,
        http: &Http,
        message_id: MessageId,
        builder: CreateInteractionResponseFollowup<'_>,
    ) -> Result<Message> {
        builder.execute(http, Some(message_id), &self.token).await
    }

    /// Deletes a followup message.
    ///
    /// # Errors
    ///
    /// May return [`Error::Http`] if the API returns an error. Such as if the response was already
    /// deleted.
    pub async fn delete_followup(&self, http: &Http, message_id: MessageId) -> Result<()> {
        http.delete_followup_message(&self.token, message_id).await
    }

    /// Helper function to defer an interaction.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the API returns an error, or an [`Error::Json`] if there is
    /// an error in deserializing the API response.
    pub async fn defer(&self, http: &Http) -> Result<()> {
        self.create_response(http, CreateInteractionResponse::Acknowledge).await
    }

    /// Helper function to defer an interaction ephemerally
    ///
    /// # Errors
    ///
    /// May also return an [`Error::Http`] if the API returns an error, or an [`Error::Json`] if
    /// there is an error in deserializing the API response.
    pub async fn defer_ephemeral(&self, http: &Http) -> Result<()> {
        let builder = CreateInteractionResponse::Defer(
            CreateInteractionResponseMessage::new().ephemeral(true),
        );
        self.create_response(http, builder).await
    }
}

// Manual impl needed to insert guild_id into resolved Role's
impl<'de> Deserialize<'de> for ModalInteraction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> StdResult<Self, D::Error> {
        let mut interaction = Self::deserialize(deserializer)?; // calls #[serde(remote)]-generated inherent method
        if let (Some(guild_id), Some(member)) = (interaction.guild_id, &mut interaction.member) {
            member.guild_id = guild_id;
            // If `member` is present, `user` wasn't sent and is still filled with default data
            interaction.user = member.user.clone();
        }
        Ok(interaction)
    }
}

impl Serialize for ModalInteraction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> StdResult<S::Ok, S::Error> {
        Self::serialize(self, serializer) // calls #[serde(remote)]-generated inherent method
    }
}

/// A modal submit interaction data, provided by [`ModalInteraction::data`]
///
/// [Discord docs](https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object-interaction-data-structure).
#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct ModalInteractionData {
    /// The custom id of the modal
    pub custom_id: FixedString,
    /// The components.
    pub components: FixedArray<ActionRow>,
}
