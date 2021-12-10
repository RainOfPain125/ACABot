# ACABot ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/RainOfPain125/ACABot/Rust) [![GitHub license](https://img.shields.io/github/license/RainOfPain125/ACABot)](https://github.com/RainOfPain125/ACABot/blob/main/LICENSE) [![GitHub issues](https://img.shields.io/github/issues/RainOfPain125/ACABot)](https://github.com/RainOfPain125/ACABot/issues) [![GitHub stars](https://img.shields.io/github/stars/RainOfPain125/ACABot)](https://github.com/RainOfPain125/ACABot/stargazers) ![Lines of code](https://img.shields.io/tokei/lines/github.com/RainOfPain125/ACABot) ![Discord](https://img.shields.io/discord/714625798056706057)
A Discord bot focused on addressing the inherent problems with Discord, to allow a more socialist/anarchist organization of servers (or "guilds").

PLEASE help contribute to the bot in our development Discord! https://discord.gg/RRkVZcw
The task is up to someone or a team of folks, to collaborate together and see it through that the bot is created and running.
I personally can not code, but I would love to see this in action. For the betterment of all socialist & anarchist communities.

Bot functions outline https://docs.google.com/document/d/12JEqEaEH5LSH_n49ro7QveMJdKs_ibxGFkQWb1194P0/edit?usp=sharing

If anything below sounds unnecessary, then it is most likely an option to edit or disable it.

- # Vetting
  - Auto create new channels for each person who begins vetting.
  - Create custom commands to accept user applications.
  - Auto archive and delete vetting chats after user is accepted.
  - Sync vetting questions across linked servers.

- # Moderation
  - Typical mod things.
  - Add custom "signature" to all ban messages (such as a link to a ban appeal website/discord).
  - Ban messages notify user who banned them, for what reason, from what server.
  - Passive anti-role nuke. Allow chosen users to create/edit roles, but not delete them.
  - Passive anti-chat nuke. Allow chosen users to create/edit chats, but not delete them.
  - Passive anti-ban nuke. If a moderator bans too many people in a short amount of time, the bot will remove their moderator role and notify the rest of the mods.
  - Easy system for mass-banning from a provided list user IDs. (admins only!)

- # Decision Making
  - Better voting methods than FPTP (Score, Range, IRV, Runoff, so on).
  - First-time setup notify via DM all vetted users about the bot and how to use it (done via command). Will also ask the user if they would like future notifications on new votes.
  - Auto create new channels for each new vote.
  - Simple, easy, and anonymous way to create votes via DMs.
  - Simple, easy, and anonymous way to cast your vote via DMs.
  - Cross-server voting via DMs.
  - "Liquid democracy". Delegate your voting power to another user, to be taken back at any time. (Automatically taken back if user and delegated user submit their choice for the same vote).

- # Syncing
  - Sync bans across servers (either all bans, or only bans done through a special ban command).
  - Sync roles across servers (Such as pronouns, chat color, etc. If you trust the vetting process of a linked server, then you can also sync the approved vetting role so that if the user is accepted in their discord, they will automatically be accepted in yours).
  - As said earlier, sync vetting questions across linked servers.
  - Sync mod logs across linked servers.
