use serenity::all::CommandDataOptionValue;
use serenity::all::CommandInteraction;
use serenity::all::CommandOptionType;
use serenity::builder::CreateCommand;
use serenity::builder::CreateCommandOption;
use serenity::builder::CreateEmbed;
use serenity::builder::CreateEmbedFooter;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::client::Context;

use crate::api::qnaplus::schema::*;
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
        let rule = if let CommandDataOptionValue::String(arg) = &interaction.data.options[0].value {
            arg.trim()
        } else {
            return CreateInteractionResponseMessage::new().content("No argument provided");
        };

        let season = match interaction.data.options.get(1) {
            Some(option) => option.value.as_str(),
            None => None,
        };

        let embed = match qnaplus.get_qnas_for_rule(rule, season).await {
            Ok(response) => match response {
                RuleResponse::RuleError(error) => {
                    CreateEmbed::new().title("API Error").description(format!(
                        "{}```txt\n{}```",
                        error.message,
                        error
                            .error
                            .unwrap_or("[no error stacktrace provided]".to_string())
                    ))
                }
                RuleResponse::Rule(rule) => {
                    let Rule {
                        rule,
                        description,
                        link,
                        questions,
                    } = rule;
                    let questions = questions
                        .iter()
                        .map(|q| {
                            format!("{} - [{}]({})\n", &q.author, &q.title, &q.url).to_string()
                        })
                        .collect::<Vec<_>>()
                        .join("");

                    CreateEmbed::new()
                        .description(format!(
                            "## [{rule}]({link})\n{description}\n\n**Related Questions:**\n{questions}",
                        ))
                        .footer(CreateEmbedFooter::new(
                            "Data provided by qnapl.us and referee.fyi",
                        ))
                }
            },
            Err(err) => CreateEmbed::new()
                .title("Failed to fetch data from qnaplus.")
                .description(format!("```rs\n{err:?}```")),
        };

        CreateInteractionResponseMessage::new().add_embed(embed)
    }
}
