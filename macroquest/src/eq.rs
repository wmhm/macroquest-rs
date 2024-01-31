use std::fmt;

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::ffi;

#[derive(Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum GameState {
    PreCharacterSelect = -1,
    CharacterSelect = 1,
    CharacterCreate = 2,
    PostCharacterSelect = 3,
    InGame = 5,
    LoggingIn = 253,
    Unloading = 255,

    #[num_enum(catch_all)]
    Unknown(i32),
}

#[derive(Debug, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum ChatColor {
    Say = chat_color!(1),
    Tell = chat_color!(2),
    Group = chat_color!(3),
    Guild = chat_color!(4),
    OOC = chat_color!(5),
    Auction = chat_color!(6),
    Shout = chat_color!(7),
    Emote = chat_color!(8),
    Spells = chat_color!(9),
    YouHitOther = chat_color!(10),
    OtherHitsYou = chat_color!(11),
    YouMissOther = chat_color!(12),
    OtherMissYou = chat_color!(13),
    Duels = chat_color!(14),
    Skills = chat_color!(15),
    Disciplines = chat_color!(16),
    // Unused = chat_color!(17),
    Default = chat_color!(18),
    Faction = chat_color!(19),
    MerchantOffer = chat_color!(20),
    MerchantExchange = chat_color!(21),
    YourDeath = chat_color!(22),
    OtherDeath = chat_color!(23),
    OtherHitOther = chat_color!(24),
    OtherMissOther = chat_color!(25),
    Who = chat_color!(26),
    Yell = chat_color!(27),
    SpellDamage = chat_color!(28),
    SpellWornOff = chat_color!(29),
    MoneySplit = chat_color!(30),
    Loot = chat_color!(31),
    Random = chat_color!(32),
    OthersSpells = chat_color!(33),
    SpellFailure = chat_color!(34),
    ChatChannel = chat_color!(35),
    ChatChannel1 = chat_color!(36),
    ChatChannel2 = chat_color!(37),
    ChatChannel3 = chat_color!(38),
    ChatChannel4 = chat_color!(39),
    ChatChannel5 = chat_color!(40),
    ChatChannel6 = chat_color!(41),
    ChatChannel7 = chat_color!(42),
    ChatChannel8 = chat_color!(43),
    ChatChannel9 = chat_color!(44),
    ChatChannel10 = chat_color!(45),
    MeleeCrit = chat_color!(46),
    SpellCrit = chat_color!(47),
    TooFarAway = chat_color!(48),
    NPCRampage = chat_color!(49),
    NPCFlurry = chat_color!(50),
    NPCEnrage = chat_color!(51),
    EchoSay = chat_color!(52),
    EchoTell = chat_color!(53),
    EchoGroup = chat_color!(54),
    EchoGuild = chat_color!(55),
    EchoOOC = chat_color!(56),
    EchoAuction = chat_color!(57),
    EchoShout = chat_color!(58),
    EchoEmote = chat_color!(59),
    EchoChatChannel1 = chat_color!(60),
    EchoChatChannel2 = chat_color!(61),
    EchoChatChannel3 = chat_color!(62),
    EchoChatChannel4 = chat_color!(63),
    EchoChatChannel5 = chat_color!(64),
    EchoChatChannel6 = chat_color!(65),
    EchoChatChannel7 = chat_color!(66),
    EchoChatChannel8 = chat_color!(67),
    EchoChatChannel9 = chat_color!(68),
    EchoChatChannel10 = chat_color!(69),
    AvatarCommand = chat_color!(70),
    Link = chat_color!(71),
    Raid = chat_color!(72),
    Pet = chat_color!(73),
    DamageShield = chat_color!(74),
    Leader = chat_color!(75),
    PetRampageFlurry = chat_color!(76),
    PetCrit = chat_color!(77),
    Focus = chat_color!(78),
    Experience = chat_color!(79),
    System = chat_color!(80),
    PetSpells = chat_color!(81),
    PetResponses = chat_color!(82),
    ItemSpeech = chat_color!(83),
    Strikethrough = chat_color!(84),
    Stun = chat_color!(85),
    SwarmPetDeath = chat_color!(86),
    Fellowship = chat_color!(87),
    NPCSpeech = chat_color!(88),
    NPCSpeechToYou = chat_color!(89),
    GuildMessage = chat_color!(90),
    MercenaryGroup = chat_color!(91),
    Achievement = chat_color!(92),
    AchievementYou = chat_color!(93),
    AchievementOthers = chat_color!(94),
    PvP = chat_color!(95),
    HotButtonCooldown = chat_color!(96),
    AggroLow = chat_color!(97),
    AggroWarning = chat_color!(98),
    AggroMost = chat_color!(99),
    DialogLink = chat_color!(100),
    YouFlurry = chat_color!(101),
    Debug = chat_color!(102),
    NPCDeath = chat_color!(103),
    RandomOther = chat_color!(104),
    RandomGroup = chat_color!(105),
    YouFallDamage = chat_color!(106),
    OtherFallDamage = chat_color!(107),
    YouDamageSield = chat_color!(108),
    OtherDamageShield = chat_color!(109),
    Event = chat_color!(110),
    DetrimentalSpellOverwritten = chat_color!(111),
    BeneficialSpellOverwritten = chat_color!(112),
    CantUseCommand = chat_color!(113),
    AbilityCooldown = chat_color!(114),
    AltAbilityCooldown = chat_color!(115),
    DestroyItem = chat_color!(116),
    AurasYours = chat_color!(117),
    AurasOthers = chat_color!(118),
    HealsYours = chat_color!(119),
    HealsOthers = chat_color!(120),
    DoTsYours = chat_color!(121),
    DoTsOthers = chat_color!(122),
    PetBardSongs = chat_color!(123),
    DirectDamageOthers = chat_color!(124),
    SpellEmotes = chat_color!(125),
    FactionLink = chat_color!(126),
    Taunt = chat_color!(127),
    DisciplinesOthers = chat_color!(128),
    ItemStatPositive = chat_color!(129),
    ItemStatNegative = chat_color!(130),
    EncounterLockAttackable = chat_color!(131),
    EncounterLockUnattackable = chat_color!(132),
    FoodAndDrink = chat_color!(133),
    RaidVictory = chat_color!(134),
    DirectDamageYours = chat_color!(142),
    DirectDamageOthersCrit = chat_color!(143),
    DoTsYoursCrit = chat_color!(144),
    DoTsOthersCrit = chat_color!(145),
    DoTsDamageTaken = chat_color!(146),
    HealsReceived = chat_color!(147),
    HealsYoursCrit = chat_color!(148),
    HealsOthersCrit = chat_color!(149),
    MeleeOthersCrit = chat_color!(150),

    #[num_enum(catch_all)]
    Unknown(i32),
}

pub struct Spawn<'a>(pub(crate) &'a ffi::eqlib::PlayerClient);

impl<'a> Spawn<'a> {
    getter!(name -> &str);
}

impl<'a> fmt::Debug for Spawn<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spawn").field("name", &self.name()).finish()
    }
}

pub struct GroundItem<'a>(pub(crate) &'a ffi::eqlib::EQGroundItem);

impl<'a> GroundItem<'a> {
    getter!(name -> &str);
}

impl<'a> fmt::Debug for GroundItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroundItem")
            .field("name", &self.name())
            .finish()
    }
}

mod macros {
    macro_rules! chat_color {
        ($num:literal) => {
            $num + 255
        };
    }

    macro_rules! getter {
        ($name:ident -> $rtype:ty) => {
            #[must_use]
            pub fn $name(&self) -> $rtype {
                self.0.$name()
            }
        };
    }

    pub(super) use {chat_color, getter};
}

use macros::{chat_color, getter};
