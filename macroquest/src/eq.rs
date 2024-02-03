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

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ChatColor {
    // All ChatColor members start at 256, but are logically starting from 1
    Say                = 255 + 1,
    Tell               = 255 + 2,
    Group              = 255 + 3,
    Guild              = 255 + 4,
    OOC                = 255 + 5,
    Auction            = 255 + 6,
    Shout              = 255 + 7,
    Emote              = 255 + 8,
    Spells             = 255 + 9,
    YouHitOther        = 255 + 10,
    OtherHitsYou       = 255 + 11,
    YouMissOther       = 255 + 12,
    OtherMissYou       = 255 + 13,
    Duels              = 255 + 14,
    Skills             = 255 + 15,
    Disciplines        = 255 + 16,
    Default            = 255 + 18,
    Faction            = 255 + 19,
    MerchantOffer      = 255 + 20,
    MerchantExchange   = 255 + 21,
    YourDeath          = 255 + 22,
    OtherDeath         = 255 + 23,
    OtherHitOther      = 255 + 24,
    OtherMissOther     = 255 + 25,
    Who                = 255 + 26,
    Yell               = 255 + 27,
    SpellDamage        = 255 + 28,
    SpellWornOff       = 255 + 29,
    MoneySplit         = 255 + 30,
    Loot               = 255 + 31,
    Random             = 255 + 32,
    OthersSpells       = 255 + 33,
    SpellFailure       = 255 + 34,
    ChatChannel        = 255 + 35,
    ChatChannel1       = 255 + 36,
    ChatChannel2       = 255 + 37,
    ChatChannel3       = 255 + 38,
    ChatChannel4       = 255 + 39,
    ChatChannel5       = 255 + 40,
    ChatChannel6       = 255 + 41,
    ChatChannel7       = 255 + 42,
    ChatChannel8       = 255 + 43,
    ChatChannel9       = 255 + 44,
    ChatChannel10      = 255 + 45,
    MeleeCrit          = 255 + 46,
    SpellCrit          = 255 + 47,
    TooFarAway         = 255 + 48,
    NPCRampage         = 255 + 49,
    NPCFlurry          = 255 + 50,
    NPCEnrage          = 255 + 51,
    EchoSay            = 255 + 52,
    EchoTell           = 255 + 53,
    EchoGroup          = 255 + 54,
    EchoGuild          = 255 + 55,
    EchoOOC            = 255 + 56,
    EchoAuction        = 255 + 57,
    EchoShout          = 255 + 58,
    EchoEmote          = 255 + 59,
    EchoChatChannel1   = 255 + 60,
    EchoChatChannel2   = 255 + 61,
    EchoChatChannel3   = 255 + 62,
    EchoChatChannel4   = 255 + 63,
    EchoChatChannel5   = 255 + 64,
    EchoChatChannel6   = 255 + 65,
    EchoChatChannel7   = 255 + 66,
    EchoChatChannel8   = 255 + 67,
    EchoChatChannel9   = 255 + 68,
    EchoChatChannel10  = 255 + 69,
    AvatarCommand      = 255 + 70,
    Link               = 255 + 71,
    Raid               = 255 + 72,
    Pet                = 255 + 73,
    DamageShield       = 255 + 74,
    Leader             = 255 + 75,
    PetRampageFlurry   = 255 + 76,
    PetCrit            = 255 + 77,
    Focus              = 255 + 78,
    Experience         = 255 + 79,
    System             = 255 + 80,
    PetSpells          = 255 + 81,
    PetResponses       = 255 + 82,
    ItemSpeech         = 255 + 83,
    Strikethrough      = 255 + 84,
    Stun               = 255 + 85,
    SwarmPetDeath      = 255 + 86,
    Fellowship         = 255 + 87,
    NPCSpeech          = 255 + 88,
    NPCSpeechToYou     = 255 + 89,
    GuildMessage       = 255 + 90,
    MercenaryGroup     = 255 + 91,
    Achievement        = 255 + 92,
    AchievementYou     = 255 + 93,
    AchievementOthers  = 255 + 94,
    PvP                = 255 + 95,
    HotButtonCooldown  = 255 + 96,
    AggroLow           = 255 + 97,
    AggroWarning       = 255 + 98,
    AggroMost          = 255 + 99,
    DialogLink         = 255 + 100,
    YouFlurry          = 255 + 101,
    Debug              = 255 + 102,
    NPCDeath           = 255 + 103,
    RandomOther        = 255 + 104,
    RandomGroup        = 255 + 105,
    YouFallDamage      = 255 + 106,
    OtherFallDamage    = 255 + 107,
    YouDamageSield     = 255 + 108,
    OtherDamageShield  = 255 + 109,
    Event              = 255 + 110,
    DetrimentalSpellOverwritten = 255 + 111,
    BeneficialSpellOverwritten = 255 + 112,
    CantUseCommand     = 255 + 113,
    AbilityCooldown    = 255 + 114,
    AltAbilityCooldown = 255 + 115,
    DestroyItem        = 255 + 116,
    AurasYours         = 255 + 117,
    AurasOthers        = 255 + 118,
    HealsYours         = 255 + 119,
    HealsOthers        = 255 + 120,
    DoTsYours          = 255 + 121,
    DoTsOthers         = 255 + 122,
    PetBardSongs       = 255 + 123,
    DirectDamageOthers = 255 + 124,
    SpellEmotes        = 255 + 125,
    FactionLink        = 255 + 126,
    Taunt              = 255 + 127,
    DisciplinesOthers  = 255 + 128,
    ItemStatPositive   = 255 + 129,
    ItemStatNegative   = 255 + 130,
    EncounterLockAttackable = 255 + 131,
    EncounterLockUnattackable = 255 + 132,
    FoodAndDrink       = 255 + 133,
    RaidVictory        = 255 + 134,
    DirectDamageYours  = 255 + 142,
    DirectDamageOthersCrit = 255 + 143,
    DoTsYoursCrit      = 255 + 144,
    DoTsOthersCrit     = 255 + 145,
    DoTsDamageTaken    = 255 + 146,
    HealsReceived      = 255 + 147,
    HealsYoursCrit     = 255 + 148,
    HealsOthersCrit    = 255 + 149,
    MeleeOthersCrit    = 255 + 150,

    #[num_enum(catch_all)]
    Unknown(i32),
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
