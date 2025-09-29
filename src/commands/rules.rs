use serenity::all::CommandInteraction;
use serenity::all::CommandOptionType;
use serenity::all::CreateCommandOption;
use serenity::all::CreateEmbed;
use serenity::builder::CreateCommand;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::client::Context;

use crate::api::qnaplus::Qnaplus;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct RulesCommand;

impl RulesCommand {
    pub fn command() -> CreateCommand {
        CreateCommand::new("rules")
            .description("Search for rules and related Q&As.")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "name",
                    "The rule to retrieve, e.g., G4, SG11.",
                )
                .required(true),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "season",
                    "The season to retrieve the rule from. Defaults to the current season.",
                )
                .required(false),
            )
    }

    pub async fn response(
        &self,
        _ctx: &Context,
        interaction: &CommandInteraction,
        qnaplus: &Qnaplus,
    ) -> CreateInteractionResponseMessage {
        CreateInteractionResponseMessage::new().add_embed(CreateEmbed::new().title("Pong!"))
    }
}
