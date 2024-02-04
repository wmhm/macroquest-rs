//!

use std::fmt;

use num_enum::{FromPrimitive, IntoPrimitive};
use ref_cast::RefCast;

use crate::ffi;

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GameState {
    PreCharacterSelect  = -1,
    CharacterSelect     = 1,
    CharacterCreate     = 2,
    PostCharacterSelect = 3,
    InGame              = 5,
    LoggingIn           = 253,
    Unloading           = 255,

    #[num_enum(catch_all)]
    Unknown(i32),
}

/// Represents the various types of chat "channels".
///
/// Messages in EverQuest get emitted to a specific channel, some of these
/// channels are channels that users can talk on (like say, or group) others
/// are channels that only the game itself can send messages on (like skills).
///
/// MacroQuest/EverQuest calls these the "color" of the chat message, because
/// they control which filter the message applies under (and thus what "color"
/// it gets printed with).
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum Channel {
    /// Say
    Say                = 255 + 1,
    /// Tell
    Tell               = 255 + 2,
    /// Group
    Group              = 255 + 3,
    /// Guild
    Guild              = 255 + 4,
    /// Out Of Character (OOC)
    OOC                = 255 + 5,
    /// Auction
    Auction            = 255 + 6,
    /// Shout
    Shout              = 255 + 7,
    /// Emotes
    Emote              = 255 + 8,
    /// Spells - Memming, scribing, casting, etc
    Spells             = 255 + 9,
    /// You Hit others
    YouHitOther        = 255 + 10,
    /// Other hits you
    OtherHitsYou       = 255 + 11,
    /// You miss other
    YouMissOther       = 255 + 12,
    /// Other misses you
    OtherMissYou       = 255 + 13,
    /// Broadcasts (duels, etc)
    Duels              = 255 + 14,
    /// Skills - Skill ups, non-combat use, etc
    Skills             = 255 + 15,
    /// Disciplines or special abilities
    Disciplines        = 255 + 16,
    /// Default text and stuff you type
    Default            = 255 + 18,
    /// Faction Messages
    Faction            = 255 + 19,
    /// Merchant Offer Price
    MerchantOffer      = 255 + 20,
    /// Merchant Buy/Sell
    MerchantExchange   = 255 + 21,
    /// Your death message
    YourDeath          = 255 + 22,
    /// Others death message
    OtherDeath         = 255 + 23,
    /// Other damage other
    OtherHitOther      = 255 + 24,
    /// Other miss other
    OtherMissOther     = 255 + 25,
    /// /who command
    Who                = 255 + 26,
    /// Yell for help
    Yell               = 255 + 27,
    /// Spell Damage
    SpellDamage        = 255 + 28,
    /// Spell worn off
    SpellWornOff       = 255 + 29,
    /// Money splits
    MoneySplit         = 255 + 30,
    /// Loot message
    Loot               = 255 + 31,
    /// Dice Roll (/random)
    Random             = 255 + 32,
    /// Others spells
    OthersSpells       = 255 + 33,
    /// Spell Failures - Resists, fizzles, missing component, bad target, etc
    SpellFailure       = 255 + 34,
    /// Chat Channel Messages
    ChatChannel        = 255 + 35,
    /// Chat Channel 1
    ChatChannel1       = 255 + 36,
    /// Chat Channel 2
    ChatChannel2       = 255 + 37,
    /// Chat Channel 3
    ChatChannel3       = 255 + 38,
    /// Chat Channel 4
    ChatChannel4       = 255 + 39,
    /// Chat Channel 5
    ChatChannel5       = 255 + 40,
    /// Chat Channel 6
    ChatChannel6       = 255 + 41,
    /// Chat Channel 7
    ChatChannel7       = 255 + 42,
    /// Chat Channel 8
    ChatChannel8       = 255 + 43,
    /// Chat Channel 9
    ChatChannel9       = 255 + 44,
    /// Chat Channel 10
    ChatChannel10      = 255 + 45,
    /// Melee Crits (Yours)
    MeleeCrit          = 255 + 46,
    /// Direct Damage Crits (Yours)
    SpellCrit          = 255 + 47,
    /// Too far away (Melee)
    TooFarAway         = 255 + 48,
    /// NPC Rampage
    NPCRampage         = 255 + 49,
    /// NPC Flurry
    NPCFlurry          = 255 + 50,
    /// NPC Enrage
    NPCEnrage          = 255 + 51,
    /// Say Echo
    EchoSay            = 255 + 52,
    /// Tell Echo
    EchoTell           = 255 + 53,
    /// Group Echo
    EchoGroup          = 255 + 54,
    /// Guild Echo
    EchoGuild          = 255 + 55,
    /// OOC Echo
    EchoOOC            = 255 + 56,
    /// Auction Echo
    EchoAuction        = 255 + 57,
    /// Shout Echo
    EchoShout          = 255 + 58,
    /// Emote Echo
    EchoEmote          = 255 + 59,
    /// Chat Channel 1 Echo
    EchoChatChannel1   = 255 + 60,
    /// Chat Channel 2 Echo
    EchoChatChannel2   = 255 + 61,
    /// Chat Channel 3 Echo
    EchoChatChannel3   = 255 + 62,
    /// Chat Channel 4 Echo
    EchoChatChannel4   = 255 + 63,
    /// Chat Channel 5 Echo
    EchoChatChannel5   = 255 + 64,
    /// Chat Channel 6 Echo
    EchoChatChannel6   = 255 + 65,
    /// Chat Channel 7 Echo
    EchoChatChannel7   = 255 + 66,
    /// Chat Channel 8 Echo
    EchoChatChannel8   = 255 + 67,
    /// Chat Channel 9 Echo
    EchoChatChannel9   = 255 + 68,
    /// Chat Channel 10 Echo
    EchoChatChannel10  = 255 + 69,
    /// Avatar Command Output
    AvatarCommand      = 255 + 70,
    /// Item Links
    Link               = 255 + 71,
    /// Raid Say
    Raid               = 255 + 72,
    /// Pet Melee (Yours)
    Pet                = 255 + 73,
    /// Damage Shield hits you
    DamageShield       = 255 + 74,
    /// Group / Raid Role messages
    Leader             = 255 + 75,
    /// Pet rampage/flurry
    PetRampageFlurry   = 255 + 76,
    /// Pet's critical hits
    PetCrit            = 255 + 77,
    /// Focus item activation
    Focus              = 255 + 78,
    /// XP Gain/Loss
    Experience         = 255 + 79,
    /// System Broadcasts
    System             = 255 + 80,
    /// Pet spells
    PetSpells          = 255 + 81,
    /// Pet responses
    PetResponses       = 255 + 82,
    /// Item Speech
    ItemSpeech         = 255 + 83,
    /// Strikethrough Messages
    Strikethrough      = 255 + 84,
    /// Stun Messages
    Stun               = 255 + 85,
    /// Swarm Pet Death
    SwarmPetDeath      = 255 + 86,
    /// Fellowship Messages
    Fellowship         = 255 + 87,
    /// NPC Dialogue
    NPCSpeech          = 255 + 88,
    /// NPC Dialogue to You
    NPCSpeechToYou     = 255 + 89,
    /// Guild Messages
    GuildMessage       = 255 + 90,
    /// Mercenary to Group Messages
    MercenaryGroup     = 255 + 91,
    /// Achievement Links
    Achievement        = 255 + 92,
    /// Achievement (Yours)
    AchievementYou     = 255 + 93,
    /// Achievement (Others)
    AchievementOthers  = 255 + 94,
    /// PvP Messages
    PvP                = 255 + 95,
    /// Hotbutton Cooldown Overlay
    HotButtonCooldown  = 255 + 96,
    /// Aggro Labels - Low
    AggroLow           = 255 + 97,
    /// Aggro Labels - Warning
    AggroWarning       = 255 + 98,
    /// Aggro Labels - Most
    AggroMost          = 255 + 99,
    /// Dialog [Response] Links
    DialogLink         = 255 + 100,
    /// Flurry (Yours)
    YouFlurry          = 255 + 101,
    /// Debug Output
    Debug              = 255 + 102,
    /// Death Notification - NPCs
    NPCDeath           = 255 + 103,
    /// Dice Roll (/random) - Others
    RandomOther        = 255 + 104,
    /// Dice Roll (/random) - Group / Raid
    RandomGroup        = 255 + 105,
    /// Environmental Damage (Yours)
    YouFallDamage      = 255 + 106,
    /// Environmental Damage (Others)
    OtherFallDamage    = 255 + 107,
    /// Damage Shield (Yours)
    YouDamageSield     = 255 + 108,
    /// Damage Shield (Other)
    OtherDamageShield  = 255 + 109,
    /// Event Messages
    Event              = 255 + 110,
    /// Spell Overwritten (Detrimental)
    DetrimentalSpellOverwritten = 255 + 111,
    /// Spell Overwritten (Beneficial)
    BeneficialSpellOverwritten = 255 + 112,
    /// Can't Use Command Warning
    CantUseCommand     = 255 + 113,
    /// Combat Ability Reuse - You can use [Ability Name] again in [time until]
    AbilityCooldown    = 255 + 114,
    /// Alt Ability Reuse
    AltAbilityCooldown = 255 + 115,
    /// Destroy Item Message
    DestroyItem        = 255 + 116,
    /// Auras (You)
    AurasYours         = 255 + 117,
    /// Aura (Others)
    AurasOthers        = 255 + 118,
    /// Heals (You)
    HealsYours         = 255 + 119,
    /// Heals (Others)
    HealsOthers        = 255 + 120,
    /// DoT (Yours)
    DoTsYours          = 255 + 121,
    /// DoT (Others)
    DoTsOthers         = 255 + 122,
    /// Bard Songs on Pets
    PetBardSongs       = 255 + 123,
    /// Direct Damage (Others)
    DirectDamageOthers = 255 + 124,
    /// Spell Emotes
    SpellEmotes        = 255 + 125,
    /// Faction Links
    FactionLink        = 255 + 126,
    /// Taunt Messages
    Taunt              = 255 + 127,
    /// Combat Abilities / Disciplines (Others)
    DisciplinesOthers  = 255 + 128,
    /// Item Stat Positive
    ItemStatPositive   = 255 + 129,
    /// Item Stat Negative
    ItemStatNegative   = 255 + 130,
    /// Encounter Lock Attackable
    EncounterLockAttackable = 255 + 131,
    /// Encounter Lock Unattackable
    EncounterLockUnattackable = 255 + 132,
    /// Food And Drink
    FoodAndDrink       = 255 + 133,
    /// Raid Victory
    RaidVictory        = 255 + 134,
    /// Direct Damage (Yours)
    DirectDamageYours  = 255 + 142,
    /// Direct Damage (Other Critical Hits)
    DirectDamageOthersCrit = 255 + 143,
    /// DoTs (Your Critical Hits)
    DoTsYoursCrit      = 255 + 144,
    /// DoTs (Other Critical Hits)
    DoTsOthersCrit     = 255 + 145,
    /// DoTs (You Being Hit)
    DoTsDamageTaken    = 255 + 146,
    /// Heals Received
    HealsReceived      = 255 + 147,
    /// Heals (Your Critical Heals)
    HealsYoursCrit     = 255 + 148,
    /// Heals (Other Critical Heals)
    HealsOthersCrit    = 255 + 149,
    /// Others Hits (Critical)
    MeleeOthersCrit    = 255 + 150,

    /// Unknown Chat Color
    #[num_enum(catch_all)]
    Unknown(i32),
}

#[allow(clippy::derivable_impls)]
impl Default for Channel {
    fn default() -> Self {
        Channel::Default
    }
}

#[allow(missing_docs)]
#[derive(RefCast)]
#[repr(transparent)]
pub struct Spawn(ffi::eqlib::PlayerClient);

#[allow(missing_docs)]
impl Spawn {
    getter!(name -> &str);
}

impl AsRef<Spawn> for ffi::eqlib::PlayerClient {
    fn as_ref(&self) -> &Spawn {
        Spawn::ref_cast(self)
    }
}

impl fmt::Debug for Spawn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spawn").field("name", &self.name()).finish()
    }
}

#[allow(missing_docs)]
#[derive(RefCast)]
#[repr(transparent)]
pub struct GroundItem(ffi::eqlib::EQGroundItem);

#[allow(missing_docs)]
impl GroundItem {
    getter!(name -> &str);
}

impl AsRef<GroundItem> for ffi::eqlib::EQGroundItem {
    fn as_ref(&self) -> &GroundItem {
        GroundItem::ref_cast(self)
    }
}

impl fmt::Debug for GroundItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroundItem")
            .field("name", &self.name())
            .finish()
    }
}

mod macros {
    macro_rules! getter {
        ($name:ident -> $rtype:ty) => {
            #[must_use]
            pub fn $name(&self) -> $rtype {
                self.0.$name()
            }
        };
    }

    pub(super) use getter;
}

use macros::getter;
