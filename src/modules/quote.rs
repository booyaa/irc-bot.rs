use core::*;
use core::BotCmdAuthLvl as Auth;
use std::borrow::Cow;
use util;
use yaml_rust::Yaml;

/// This module provides functionality for retrieving quotations from a database thereof.
///
/// # The `quote` command
///
/// An IRC user is to interact with this module primarily via the bot command `quote`, which
/// requests a (pseudo-)random quotation from the bot's database of quotations.
///
/// ## Syntax
///
/// The `quote` command takes as argument a YAML mapping, which may contain the following key-value
/// pairs (hereinafter termed _parameters_), listed by their keys:
///
/// - `regex` — The value of this parameter may be a scalar or a sequence of scalars. If a scalar,
/// it will be interpreted as text representing a regular expression, which text will be parsed
/// using the Rust [`regex`] library and [its particular syntax][`regex` syntax]; if a sequence of
/// scalars, each scalar it contains will be so interpreted and parsed. A quotation will be
/// displayed only if it contains at least one match of each regular expression so provided. These
/// regular expressions will be matched case-insensitively by default; however, this can be
/// controlled with the [`regex` flag] `i`. This parameter's key may be abbreviated as `r`.
///
/// - `string` — The value of this parameter may be a scalar or a sequence of scalars. If a scalar,
/// it will be interpreted as a text value; if a sequence of scalars, each scalar it contains will
/// be so interpreted. A quotation will be displayed only if it contains at least one occurrence of
/// each text value so provided. These text values will be matched case-sensitively. This
/// parameter's key may be abbreviated as `s`.
///
/// ## Examples
///
/// ### `quote`
///
/// Request a pseudo-random quotation.
///
/// ### `quote s: rabbit`
///
/// Request a pseudo-random quotation that contains the text "rabbit".
///
/// ### `quote r: 'blue ?berr(y|ies)'`
///
/// Request a pseudo-random quotation that contains at least one of the following sequences of
/// text (without regard to letter case):
///
/// - "blueberry"
/// - "blue berry"
/// - "blueberries"
/// - "blue berries"
///
/// [`regex`]: <https://docs.rs/regex/*/regex/>
/// [`regex` syntax]: <https://docs.rs/regex/*/regex/#syntax>
/// [`regex` flag]: <https://docs.rs/regex/*/regex/#grouping-and-flags>
pub fn mk() -> Module {
    mk_module("quote")
        .command(
            "quote",
            "{}",
            "Request a quotation from the bot's database of quotations. For usage instructions, \
             see the full documentation: \
             <https://docs.rs/irc-bot/*/irc_bot/modules/fn.quote.html>.",
            Auth::Public,
            Box::new(quote),
            &[],
        )
        .end()
}

fn quote(_: &State, _: &MsgMetadata, _: &Yaml) -> Reaction {
    Reaction::None
}
