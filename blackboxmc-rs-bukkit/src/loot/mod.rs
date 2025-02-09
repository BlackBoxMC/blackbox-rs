#![allow(deprecated)]
use blackboxmc_general::JNIInstantiatable;
use blackboxmc_general::JNIRaw;
use color_eyre::eyre::Result;
pub enum LootTables<'mc> {
    Empty { inner: LootTablesStruct<'mc> },
    AbandonedMineshaft { inner: LootTablesStruct<'mc> },
    BuriedTreasure { inner: LootTablesStruct<'mc> },
    DesertPyramid { inner: LootTablesStruct<'mc> },
    EndCityTreasure { inner: LootTablesStruct<'mc> },
    IglooChest { inner: LootTablesStruct<'mc> },
    JungleTemple { inner: LootTablesStruct<'mc> },
    JungleTempleDispenser { inner: LootTablesStruct<'mc> },
    NetherBridge { inner: LootTablesStruct<'mc> },
    PillagerOutpost { inner: LootTablesStruct<'mc> },
    BastionTreasure { inner: LootTablesStruct<'mc> },
    BastionOther { inner: LootTablesStruct<'mc> },
    BastionBridge { inner: LootTablesStruct<'mc> },
    BastionHoglinStable { inner: LootTablesStruct<'mc> },
    AncientCity { inner: LootTablesStruct<'mc> },
    AncientCityIceBox { inner: LootTablesStruct<'mc> },
    RuinedPortal { inner: LootTablesStruct<'mc> },
    TrialChambersReward { inner: LootTablesStruct<'mc> },
    TrialChambersRewardCommon { inner: LootTablesStruct<'mc> },
    TrialChambersRewardRare { inner: LootTablesStruct<'mc> },
    TrialChambersRewardUnique { inner: LootTablesStruct<'mc> },
    TrialChambersRewardOminous { inner: LootTablesStruct<'mc> },
    TrialChambersRewardOminousCommon { inner: LootTablesStruct<'mc> },
    TrialChambersRewardOminousRare { inner: LootTablesStruct<'mc> },
    TrialChambersRewardOminousUnique { inner: LootTablesStruct<'mc> },
    TrialChambersSupply { inner: LootTablesStruct<'mc> },
    TrialChambersCorridor { inner: LootTablesStruct<'mc> },
    TrialChambersIntersection { inner: LootTablesStruct<'mc> },
    TrialChambersIntersectionBarrel { inner: LootTablesStruct<'mc> },
    TrialChambersEntrance { inner: LootTablesStruct<'mc> },
    TrialChambersCorridorDispenser { inner: LootTablesStruct<'mc> },
    TrialChambersChamberDispenser { inner: LootTablesStruct<'mc> },
    TrialChambersWaterDispenser { inner: LootTablesStruct<'mc> },
    TrialChambersCorridorPot { inner: LootTablesStruct<'mc> },
    EquipmentTrialChamber { inner: LootTablesStruct<'mc> },
    EquipmentTrialChamberRanged { inner: LootTablesStruct<'mc> },
    EquipmentTrialChamberMelee { inner: LootTablesStruct<'mc> },
    ShipwreckMap { inner: LootTablesStruct<'mc> },
    ShipwreckSupply { inner: LootTablesStruct<'mc> },
    ShipwreckTreasure { inner: LootTablesStruct<'mc> },
    SimpleDungeon { inner: LootTablesStruct<'mc> },
    SpawnBonusChest { inner: LootTablesStruct<'mc> },
    StrongholdCorridor { inner: LootTablesStruct<'mc> },
    StrongholdCrossing { inner: LootTablesStruct<'mc> },
    StrongholdLibrary { inner: LootTablesStruct<'mc> },
    UnderwaterRuinBig { inner: LootTablesStruct<'mc> },
    UnderwaterRuinSmall { inner: LootTablesStruct<'mc> },
    VillageArmorer { inner: LootTablesStruct<'mc> },
    VillageButcher { inner: LootTablesStruct<'mc> },
    VillageCartographer { inner: LootTablesStruct<'mc> },
    VillageDesertHouse { inner: LootTablesStruct<'mc> },
    VillageFisher { inner: LootTablesStruct<'mc> },
    VillageFletcher { inner: LootTablesStruct<'mc> },
    VillageMason { inner: LootTablesStruct<'mc> },
    VillagePlainsHouse { inner: LootTablesStruct<'mc> },
    VillageSavannaHouse { inner: LootTablesStruct<'mc> },
    VillageShepherd { inner: LootTablesStruct<'mc> },
    VillageSnowyHouse { inner: LootTablesStruct<'mc> },
    VillageTaigaHouse { inner: LootTablesStruct<'mc> },
    VillageTannery { inner: LootTablesStruct<'mc> },
    VillageTemple { inner: LootTablesStruct<'mc> },
    VillageToolsmith { inner: LootTablesStruct<'mc> },
    VillageWeaponsmith { inner: LootTablesStruct<'mc> },
    WoodlandMansion { inner: LootTablesStruct<'mc> },
    ArmorStand { inner: LootTablesStruct<'mc> },
    Axolotl { inner: LootTablesStruct<'mc> },
    Bat { inner: LootTablesStruct<'mc> },
    Bee { inner: LootTablesStruct<'mc> },
    Blaze { inner: LootTablesStruct<'mc> },
    Cat { inner: LootTablesStruct<'mc> },
    CaveSpider { inner: LootTablesStruct<'mc> },
    Chicken { inner: LootTablesStruct<'mc> },
    Cod { inner: LootTablesStruct<'mc> },
    Cow { inner: LootTablesStruct<'mc> },
    Creeper { inner: LootTablesStruct<'mc> },
    Dolphin { inner: LootTablesStruct<'mc> },
    Donkey { inner: LootTablesStruct<'mc> },
    Drowned { inner: LootTablesStruct<'mc> },
    ElderGuardian { inner: LootTablesStruct<'mc> },
    EnderDragon { inner: LootTablesStruct<'mc> },
    Enderman { inner: LootTablesStruct<'mc> },
    Endermite { inner: LootTablesStruct<'mc> },
    Evoker { inner: LootTablesStruct<'mc> },
    Fox { inner: LootTablesStruct<'mc> },
    Ghast { inner: LootTablesStruct<'mc> },
    Giant { inner: LootTablesStruct<'mc> },
    GlowSquid { inner: LootTablesStruct<'mc> },
    Goat { inner: LootTablesStruct<'mc> },
    Guardian { inner: LootTablesStruct<'mc> },
    Hoglin { inner: LootTablesStruct<'mc> },
    Horse { inner: LootTablesStruct<'mc> },
    Husk { inner: LootTablesStruct<'mc> },
    Illusioner { inner: LootTablesStruct<'mc> },
    IronGolem { inner: LootTablesStruct<'mc> },
    Llama { inner: LootTablesStruct<'mc> },
    MagmaCube { inner: LootTablesStruct<'mc> },
    Mooshroom { inner: LootTablesStruct<'mc> },
    Mule { inner: LootTablesStruct<'mc> },
    Ocelot { inner: LootTablesStruct<'mc> },
    Panda { inner: LootTablesStruct<'mc> },
    Parrot { inner: LootTablesStruct<'mc> },
    Phantom { inner: LootTablesStruct<'mc> },
    Pig { inner: LootTablesStruct<'mc> },
    Piglin { inner: LootTablesStruct<'mc> },
    PiglinBrute { inner: LootTablesStruct<'mc> },
    Pillager { inner: LootTablesStruct<'mc> },
    Player { inner: LootTablesStruct<'mc> },
    PolarBear { inner: LootTablesStruct<'mc> },
    Pufferfish { inner: LootTablesStruct<'mc> },
    Rabbit { inner: LootTablesStruct<'mc> },
    Ravager { inner: LootTablesStruct<'mc> },
    Salmon { inner: LootTablesStruct<'mc> },
    Shulker { inner: LootTablesStruct<'mc> },
    Silverfish { inner: LootTablesStruct<'mc> },
    Skeleton { inner: LootTablesStruct<'mc> },
    SkeletonHorse { inner: LootTablesStruct<'mc> },
    Slime { inner: LootTablesStruct<'mc> },
    SnowGolem { inner: LootTablesStruct<'mc> },
    Spider { inner: LootTablesStruct<'mc> },
    Squid { inner: LootTablesStruct<'mc> },
    Stray { inner: LootTablesStruct<'mc> },
    Strider { inner: LootTablesStruct<'mc> },
    TraderLlama { inner: LootTablesStruct<'mc> },
    TropicalFish { inner: LootTablesStruct<'mc> },
    Turtle { inner: LootTablesStruct<'mc> },
    Vex { inner: LootTablesStruct<'mc> },
    Villager { inner: LootTablesStruct<'mc> },
    Vindicator { inner: LootTablesStruct<'mc> },
    WanderingTrader { inner: LootTablesStruct<'mc> },
    Witch { inner: LootTablesStruct<'mc> },
    Wither { inner: LootTablesStruct<'mc> },
    WitherSkeleton { inner: LootTablesStruct<'mc> },
    Wolf { inner: LootTablesStruct<'mc> },
    Zoglin { inner: LootTablesStruct<'mc> },
    Zombie { inner: LootTablesStruct<'mc> },
    ZombieHorse { inner: LootTablesStruct<'mc> },
    ZombieVillager { inner: LootTablesStruct<'mc> },
    ZombifiedPiglin { inner: LootTablesStruct<'mc> },
    ArmorerGift { inner: LootTablesStruct<'mc> },
    ButcherGift { inner: LootTablesStruct<'mc> },
    CartographerGift { inner: LootTablesStruct<'mc> },
    CatMorningGift { inner: LootTablesStruct<'mc> },
    ClericGift { inner: LootTablesStruct<'mc> },
    FarmerGift { inner: LootTablesStruct<'mc> },
    FishermanGift { inner: LootTablesStruct<'mc> },
    Fishing { inner: LootTablesStruct<'mc> },
    FishingFish { inner: LootTablesStruct<'mc> },
    FishingJunk { inner: LootTablesStruct<'mc> },
    FishingTreasure { inner: LootTablesStruct<'mc> },
    FletcherGift { inner: LootTablesStruct<'mc> },
    LeatherworkerGift { inner: LootTablesStruct<'mc> },
    LibrarianGift { inner: LootTablesStruct<'mc> },
    MasonGift { inner: LootTablesStruct<'mc> },
    ShepherdGift { inner: LootTablesStruct<'mc> },
    ToolsmithGift { inner: LootTablesStruct<'mc> },
    WeaponsmithGift { inner: LootTablesStruct<'mc> },
    SnifferDigging { inner: LootTablesStruct<'mc> },
    PandaSneeze { inner: LootTablesStruct<'mc> },
    PiglinBartering { inner: LootTablesStruct<'mc> },
    TrialChamberKey { inner: LootTablesStruct<'mc> },
    TrialChamberConsumables { inner: LootTablesStruct<'mc> },
    OminousTrialChamberKey { inner: LootTablesStruct<'mc> },
    OminousTrialChamberConsumables { inner: LootTablesStruct<'mc> },
    TrialChamberItemsToDropWhenOminous { inner: LootTablesStruct<'mc> },
    ShearingBogged { inner: LootTablesStruct<'mc> },
    DesertWellArchaeology { inner: LootTablesStruct<'mc> },
    DesertPyramidArchaeology { inner: LootTablesStruct<'mc> },
    TrailRuinsArchaeologyCommon { inner: LootTablesStruct<'mc> },
    TrailRuinsArchaeologyRare { inner: LootTablesStruct<'mc> },
    OceanRuinWarmArchaeology { inner: LootTablesStruct<'mc> },
    OceanRuinColdArchaeology { inner: LootTablesStruct<'mc> },
    Sheep { inner: LootTablesStruct<'mc> },
    SheepBlack { inner: LootTablesStruct<'mc> },
    SheepBlue { inner: LootTablesStruct<'mc> },
    SheepBrown { inner: LootTablesStruct<'mc> },
    SheepCyan { inner: LootTablesStruct<'mc> },
    SheepGray { inner: LootTablesStruct<'mc> },
    SheepGreen { inner: LootTablesStruct<'mc> },
    SheepLightBlue { inner: LootTablesStruct<'mc> },
    SheepLightGray { inner: LootTablesStruct<'mc> },
    SheepLime { inner: LootTablesStruct<'mc> },
    SheepMagenta { inner: LootTablesStruct<'mc> },
    SheepOrange { inner: LootTablesStruct<'mc> },
    SheepPink { inner: LootTablesStruct<'mc> },
    SheepPurple { inner: LootTablesStruct<'mc> },
    SheepRed { inner: LootTablesStruct<'mc> },
    SheepWhite { inner: LootTablesStruct<'mc> },
    SheepYellow { inner: LootTablesStruct<'mc> },
}
impl<'mc> std::fmt::Display for LootTables<'mc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LootTables::Empty { .. } => f.write_str("EMPTY"),
            LootTables::AbandonedMineshaft { .. } => f.write_str("ABANDONED_MINESHAFT"),
            LootTables::BuriedTreasure { .. } => f.write_str("BURIED_TREASURE"),
            LootTables::DesertPyramid { .. } => f.write_str("DESERT_PYRAMID"),
            LootTables::EndCityTreasure { .. } => f.write_str("END_CITY_TREASURE"),
            LootTables::IglooChest { .. } => f.write_str("IGLOO_CHEST"),
            LootTables::JungleTemple { .. } => f.write_str("JUNGLE_TEMPLE"),
            LootTables::JungleTempleDispenser { .. } => f.write_str("JUNGLE_TEMPLE_DISPENSER"),
            LootTables::NetherBridge { .. } => f.write_str("NETHER_BRIDGE"),
            LootTables::PillagerOutpost { .. } => f.write_str("PILLAGER_OUTPOST"),
            LootTables::BastionTreasure { .. } => f.write_str("BASTION_TREASURE"),
            LootTables::BastionOther { .. } => f.write_str("BASTION_OTHER"),
            LootTables::BastionBridge { .. } => f.write_str("BASTION_BRIDGE"),
            LootTables::BastionHoglinStable { .. } => f.write_str("BASTION_HOGLIN_STABLE"),
            LootTables::AncientCity { .. } => f.write_str("ANCIENT_CITY"),
            LootTables::AncientCityIceBox { .. } => f.write_str("ANCIENT_CITY_ICE_BOX"),
            LootTables::RuinedPortal { .. } => f.write_str("RUINED_PORTAL"),
            LootTables::TrialChambersReward { .. } => f.write_str("TRIAL_CHAMBERS_REWARD"),
            LootTables::TrialChambersRewardCommon { .. } => {
                f.write_str("TRIAL_CHAMBERS_REWARD_COMMON")
            }
            LootTables::TrialChambersRewardRare { .. } => f.write_str("TRIAL_CHAMBERS_REWARD_RARE"),
            LootTables::TrialChambersRewardUnique { .. } => {
                f.write_str("TRIAL_CHAMBERS_REWARD_UNIQUE")
            }
            LootTables::TrialChambersRewardOminous { .. } => {
                f.write_str("TRIAL_CHAMBERS_REWARD_OMINOUS")
            }
            LootTables::TrialChambersRewardOminousCommon { .. } => {
                f.write_str("TRIAL_CHAMBERS_REWARD_OMINOUS_COMMON")
            }
            LootTables::TrialChambersRewardOminousRare { .. } => {
                f.write_str("TRIAL_CHAMBERS_REWARD_OMINOUS_RARE")
            }
            LootTables::TrialChambersRewardOminousUnique { .. } => {
                f.write_str("TRIAL_CHAMBERS_REWARD_OMINOUS_UNIQUE")
            }
            LootTables::TrialChambersSupply { .. } => f.write_str("TRIAL_CHAMBERS_SUPPLY"),
            LootTables::TrialChambersCorridor { .. } => f.write_str("TRIAL_CHAMBERS_CORRIDOR"),
            LootTables::TrialChambersIntersection { .. } => {
                f.write_str("TRIAL_CHAMBERS_INTERSECTION")
            }
            LootTables::TrialChambersIntersectionBarrel { .. } => {
                f.write_str("TRIAL_CHAMBERS_INTERSECTION_BARREL")
            }
            LootTables::TrialChambersEntrance { .. } => f.write_str("TRIAL_CHAMBERS_ENTRANCE"),
            LootTables::TrialChambersCorridorDispenser { .. } => {
                f.write_str("TRIAL_CHAMBERS_CORRIDOR_DISPENSER")
            }
            LootTables::TrialChambersChamberDispenser { .. } => {
                f.write_str("TRIAL_CHAMBERS_CHAMBER_DISPENSER")
            }
            LootTables::TrialChambersWaterDispenser { .. } => {
                f.write_str("TRIAL_CHAMBERS_WATER_DISPENSER")
            }
            LootTables::TrialChambersCorridorPot { .. } => {
                f.write_str("TRIAL_CHAMBERS_CORRIDOR_POT")
            }
            LootTables::EquipmentTrialChamber { .. } => f.write_str("EQUIPMENT_TRIAL_CHAMBER"),
            LootTables::EquipmentTrialChamberRanged { .. } => {
                f.write_str("EQUIPMENT_TRIAL_CHAMBER_RANGED")
            }
            LootTables::EquipmentTrialChamberMelee { .. } => {
                f.write_str("EQUIPMENT_TRIAL_CHAMBER_MELEE")
            }
            LootTables::ShipwreckMap { .. } => f.write_str("SHIPWRECK_MAP"),
            LootTables::ShipwreckSupply { .. } => f.write_str("SHIPWRECK_SUPPLY"),
            LootTables::ShipwreckTreasure { .. } => f.write_str("SHIPWRECK_TREASURE"),
            LootTables::SimpleDungeon { .. } => f.write_str("SIMPLE_DUNGEON"),
            LootTables::SpawnBonusChest { .. } => f.write_str("SPAWN_BONUS_CHEST"),
            LootTables::StrongholdCorridor { .. } => f.write_str("STRONGHOLD_CORRIDOR"),
            LootTables::StrongholdCrossing { .. } => f.write_str("STRONGHOLD_CROSSING"),
            LootTables::StrongholdLibrary { .. } => f.write_str("STRONGHOLD_LIBRARY"),
            LootTables::UnderwaterRuinBig { .. } => f.write_str("UNDERWATER_RUIN_BIG"),
            LootTables::UnderwaterRuinSmall { .. } => f.write_str("UNDERWATER_RUIN_SMALL"),
            LootTables::VillageArmorer { .. } => f.write_str("VILLAGE_ARMORER"),
            LootTables::VillageButcher { .. } => f.write_str("VILLAGE_BUTCHER"),
            LootTables::VillageCartographer { .. } => f.write_str("VILLAGE_CARTOGRAPHER"),
            LootTables::VillageDesertHouse { .. } => f.write_str("VILLAGE_DESERT_HOUSE"),
            LootTables::VillageFisher { .. } => f.write_str("VILLAGE_FISHER"),
            LootTables::VillageFletcher { .. } => f.write_str("VILLAGE_FLETCHER"),
            LootTables::VillageMason { .. } => f.write_str("VILLAGE_MASON"),
            LootTables::VillagePlainsHouse { .. } => f.write_str("VILLAGE_PLAINS_HOUSE"),
            LootTables::VillageSavannaHouse { .. } => f.write_str("VILLAGE_SAVANNA_HOUSE"),
            LootTables::VillageShepherd { .. } => f.write_str("VILLAGE_SHEPHERD"),
            LootTables::VillageSnowyHouse { .. } => f.write_str("VILLAGE_SNOWY_HOUSE"),
            LootTables::VillageTaigaHouse { .. } => f.write_str("VILLAGE_TAIGA_HOUSE"),
            LootTables::VillageTannery { .. } => f.write_str("VILLAGE_TANNERY"),
            LootTables::VillageTemple { .. } => f.write_str("VILLAGE_TEMPLE"),
            LootTables::VillageToolsmith { .. } => f.write_str("VILLAGE_TOOLSMITH"),
            LootTables::VillageWeaponsmith { .. } => f.write_str("VILLAGE_WEAPONSMITH"),
            LootTables::WoodlandMansion { .. } => f.write_str("WOODLAND_MANSION"),
            LootTables::ArmorStand { .. } => f.write_str("ARMOR_STAND"),
            LootTables::Axolotl { .. } => f.write_str("AXOLOTL"),
            LootTables::Bat { .. } => f.write_str("BAT"),
            LootTables::Bee { .. } => f.write_str("BEE"),
            LootTables::Blaze { .. } => f.write_str("BLAZE"),
            LootTables::Cat { .. } => f.write_str("CAT"),
            LootTables::CaveSpider { .. } => f.write_str("CAVE_SPIDER"),
            LootTables::Chicken { .. } => f.write_str("CHICKEN"),
            LootTables::Cod { .. } => f.write_str("COD"),
            LootTables::Cow { .. } => f.write_str("COW"),
            LootTables::Creeper { .. } => f.write_str("CREEPER"),
            LootTables::Dolphin { .. } => f.write_str("DOLPHIN"),
            LootTables::Donkey { .. } => f.write_str("DONKEY"),
            LootTables::Drowned { .. } => f.write_str("DROWNED"),
            LootTables::ElderGuardian { .. } => f.write_str("ELDER_GUARDIAN"),
            LootTables::EnderDragon { .. } => f.write_str("ENDER_DRAGON"),
            LootTables::Enderman { .. } => f.write_str("ENDERMAN"),
            LootTables::Endermite { .. } => f.write_str("ENDERMITE"),
            LootTables::Evoker { .. } => f.write_str("EVOKER"),
            LootTables::Fox { .. } => f.write_str("FOX"),
            LootTables::Ghast { .. } => f.write_str("GHAST"),
            LootTables::Giant { .. } => f.write_str("GIANT"),
            LootTables::GlowSquid { .. } => f.write_str("GLOW_SQUID"),
            LootTables::Goat { .. } => f.write_str("GOAT"),
            LootTables::Guardian { .. } => f.write_str("GUARDIAN"),
            LootTables::Hoglin { .. } => f.write_str("HOGLIN"),
            LootTables::Horse { .. } => f.write_str("HORSE"),
            LootTables::Husk { .. } => f.write_str("HUSK"),
            LootTables::Illusioner { .. } => f.write_str("ILLUSIONER"),
            LootTables::IronGolem { .. } => f.write_str("IRON_GOLEM"),
            LootTables::Llama { .. } => f.write_str("LLAMA"),
            LootTables::MagmaCube { .. } => f.write_str("MAGMA_CUBE"),
            LootTables::Mooshroom { .. } => f.write_str("MOOSHROOM"),
            LootTables::Mule { .. } => f.write_str("MULE"),
            LootTables::Ocelot { .. } => f.write_str("OCELOT"),
            LootTables::Panda { .. } => f.write_str("PANDA"),
            LootTables::Parrot { .. } => f.write_str("PARROT"),
            LootTables::Phantom { .. } => f.write_str("PHANTOM"),
            LootTables::Pig { .. } => f.write_str("PIG"),
            LootTables::Piglin { .. } => f.write_str("PIGLIN"),
            LootTables::PiglinBrute { .. } => f.write_str("PIGLIN_BRUTE"),
            LootTables::Pillager { .. } => f.write_str("PILLAGER"),
            LootTables::Player { .. } => f.write_str("PLAYER"),
            LootTables::PolarBear { .. } => f.write_str("POLAR_BEAR"),
            LootTables::Pufferfish { .. } => f.write_str("PUFFERFISH"),
            LootTables::Rabbit { .. } => f.write_str("RABBIT"),
            LootTables::Ravager { .. } => f.write_str("RAVAGER"),
            LootTables::Salmon { .. } => f.write_str("SALMON"),
            LootTables::Shulker { .. } => f.write_str("SHULKER"),
            LootTables::Silverfish { .. } => f.write_str("SILVERFISH"),
            LootTables::Skeleton { .. } => f.write_str("SKELETON"),
            LootTables::SkeletonHorse { .. } => f.write_str("SKELETON_HORSE"),
            LootTables::Slime { .. } => f.write_str("SLIME"),
            LootTables::SnowGolem { .. } => f.write_str("SNOW_GOLEM"),
            LootTables::Spider { .. } => f.write_str("SPIDER"),
            LootTables::Squid { .. } => f.write_str("SQUID"),
            LootTables::Stray { .. } => f.write_str("STRAY"),
            LootTables::Strider { .. } => f.write_str("STRIDER"),
            LootTables::TraderLlama { .. } => f.write_str("TRADER_LLAMA"),
            LootTables::TropicalFish { .. } => f.write_str("TROPICAL_FISH"),
            LootTables::Turtle { .. } => f.write_str("TURTLE"),
            LootTables::Vex { .. } => f.write_str("VEX"),
            LootTables::Villager { .. } => f.write_str("VILLAGER"),
            LootTables::Vindicator { .. } => f.write_str("VINDICATOR"),
            LootTables::WanderingTrader { .. } => f.write_str("WANDERING_TRADER"),
            LootTables::Witch { .. } => f.write_str("WITCH"),
            LootTables::Wither { .. } => f.write_str("WITHER"),
            LootTables::WitherSkeleton { .. } => f.write_str("WITHER_SKELETON"),
            LootTables::Wolf { .. } => f.write_str("WOLF"),
            LootTables::Zoglin { .. } => f.write_str("ZOGLIN"),
            LootTables::Zombie { .. } => f.write_str("ZOMBIE"),
            LootTables::ZombieHorse { .. } => f.write_str("ZOMBIE_HORSE"),
            LootTables::ZombieVillager { .. } => f.write_str("ZOMBIE_VILLAGER"),
            LootTables::ZombifiedPiglin { .. } => f.write_str("ZOMBIFIED_PIGLIN"),
            LootTables::ArmorerGift { .. } => f.write_str("ARMORER_GIFT"),
            LootTables::ButcherGift { .. } => f.write_str("BUTCHER_GIFT"),
            LootTables::CartographerGift { .. } => f.write_str("CARTOGRAPHER_GIFT"),
            LootTables::CatMorningGift { .. } => f.write_str("CAT_MORNING_GIFT"),
            LootTables::ClericGift { .. } => f.write_str("CLERIC_GIFT"),
            LootTables::FarmerGift { .. } => f.write_str("FARMER_GIFT"),
            LootTables::FishermanGift { .. } => f.write_str("FISHERMAN_GIFT"),
            LootTables::Fishing { .. } => f.write_str("FISHING"),
            LootTables::FishingFish { .. } => f.write_str("FISHING_FISH"),
            LootTables::FishingJunk { .. } => f.write_str("FISHING_JUNK"),
            LootTables::FishingTreasure { .. } => f.write_str("FISHING_TREASURE"),
            LootTables::FletcherGift { .. } => f.write_str("FLETCHER_GIFT"),
            LootTables::LeatherworkerGift { .. } => f.write_str("LEATHERWORKER_GIFT"),
            LootTables::LibrarianGift { .. } => f.write_str("LIBRARIAN_GIFT"),
            LootTables::MasonGift { .. } => f.write_str("MASON_GIFT"),
            LootTables::ShepherdGift { .. } => f.write_str("SHEPHERD_GIFT"),
            LootTables::ToolsmithGift { .. } => f.write_str("TOOLSMITH_GIFT"),
            LootTables::WeaponsmithGift { .. } => f.write_str("WEAPONSMITH_GIFT"),
            LootTables::SnifferDigging { .. } => f.write_str("SNIFFER_DIGGING"),
            LootTables::PandaSneeze { .. } => f.write_str("PANDA_SNEEZE"),
            LootTables::PiglinBartering { .. } => f.write_str("PIGLIN_BARTERING"),
            LootTables::TrialChamberKey { .. } => f.write_str("TRIAL_CHAMBER_KEY"),
            LootTables::TrialChamberConsumables { .. } => f.write_str("TRIAL_CHAMBER_CONSUMABLES"),
            LootTables::OminousTrialChamberKey { .. } => f.write_str("OMINOUS_TRIAL_CHAMBER_KEY"),
            LootTables::OminousTrialChamberConsumables { .. } => {
                f.write_str("OMINOUS_TRIAL_CHAMBER_CONSUMABLES")
            }
            LootTables::TrialChamberItemsToDropWhenOminous { .. } => {
                f.write_str("TRIAL_CHAMBER_ITEMS_TO_DROP_WHEN_OMINOUS")
            }
            LootTables::ShearingBogged { .. } => f.write_str("SHEARING_BOGGED"),
            LootTables::DesertWellArchaeology { .. } => f.write_str("DESERT_WELL_ARCHAEOLOGY"),
            LootTables::DesertPyramidArchaeology { .. } => {
                f.write_str("DESERT_PYRAMID_ARCHAEOLOGY")
            }
            LootTables::TrailRuinsArchaeologyCommon { .. } => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_COMMON")
            }
            LootTables::TrailRuinsArchaeologyRare { .. } => {
                f.write_str("TRAIL_RUINS_ARCHAEOLOGY_RARE")
            }
            LootTables::OceanRuinWarmArchaeology { .. } => {
                f.write_str("OCEAN_RUIN_WARM_ARCHAEOLOGY")
            }
            LootTables::OceanRuinColdArchaeology { .. } => {
                f.write_str("OCEAN_RUIN_COLD_ARCHAEOLOGY")
            }
            LootTables::Sheep { .. } => f.write_str("SHEEP"),
            LootTables::SheepBlack { .. } => f.write_str("SHEEP_BLACK"),
            LootTables::SheepBlue { .. } => f.write_str("SHEEP_BLUE"),
            LootTables::SheepBrown { .. } => f.write_str("SHEEP_BROWN"),
            LootTables::SheepCyan { .. } => f.write_str("SHEEP_CYAN"),
            LootTables::SheepGray { .. } => f.write_str("SHEEP_GRAY"),
            LootTables::SheepGreen { .. } => f.write_str("SHEEP_GREEN"),
            LootTables::SheepLightBlue { .. } => f.write_str("SHEEP_LIGHT_BLUE"),
            LootTables::SheepLightGray { .. } => f.write_str("SHEEP_LIGHT_GRAY"),
            LootTables::SheepLime { .. } => f.write_str("SHEEP_LIME"),
            LootTables::SheepMagenta { .. } => f.write_str("SHEEP_MAGENTA"),
            LootTables::SheepOrange { .. } => f.write_str("SHEEP_ORANGE"),
            LootTables::SheepPink { .. } => f.write_str("SHEEP_PINK"),
            LootTables::SheepPurple { .. } => f.write_str("SHEEP_PURPLE"),
            LootTables::SheepRed { .. } => f.write_str("SHEEP_RED"),
            LootTables::SheepWhite { .. } => f.write_str("SHEEP_WHITE"),
            LootTables::SheepYellow { .. } => f.write_str("SHEEP_YELLOW"),
        }
    }
}
impl<'mc> std::ops::Deref for LootTables<'mc> {
    type Target = LootTablesStruct<'mc>;
    fn deref(&self) -> &<LootTables<'mc> as std::ops::Deref>::Target {
        match self {
            LootTables::Empty { inner } => inner,
            LootTables::AbandonedMineshaft { inner } => inner,
            LootTables::BuriedTreasure { inner } => inner,
            LootTables::DesertPyramid { inner } => inner,
            LootTables::EndCityTreasure { inner } => inner,
            LootTables::IglooChest { inner } => inner,
            LootTables::JungleTemple { inner } => inner,
            LootTables::JungleTempleDispenser { inner } => inner,
            LootTables::NetherBridge { inner } => inner,
            LootTables::PillagerOutpost { inner } => inner,
            LootTables::BastionTreasure { inner } => inner,
            LootTables::BastionOther { inner } => inner,
            LootTables::BastionBridge { inner } => inner,
            LootTables::BastionHoglinStable { inner } => inner,
            LootTables::AncientCity { inner } => inner,
            LootTables::AncientCityIceBox { inner } => inner,
            LootTables::RuinedPortal { inner } => inner,
            LootTables::TrialChambersReward { inner } => inner,
            LootTables::TrialChambersRewardCommon { inner } => inner,
            LootTables::TrialChambersRewardRare { inner } => inner,
            LootTables::TrialChambersRewardUnique { inner } => inner,
            LootTables::TrialChambersRewardOminous { inner } => inner,
            LootTables::TrialChambersRewardOminousCommon { inner } => inner,
            LootTables::TrialChambersRewardOminousRare { inner } => inner,
            LootTables::TrialChambersRewardOminousUnique { inner } => inner,
            LootTables::TrialChambersSupply { inner } => inner,
            LootTables::TrialChambersCorridor { inner } => inner,
            LootTables::TrialChambersIntersection { inner } => inner,
            LootTables::TrialChambersIntersectionBarrel { inner } => inner,
            LootTables::TrialChambersEntrance { inner } => inner,
            LootTables::TrialChambersCorridorDispenser { inner } => inner,
            LootTables::TrialChambersChamberDispenser { inner } => inner,
            LootTables::TrialChambersWaterDispenser { inner } => inner,
            LootTables::TrialChambersCorridorPot { inner } => inner,
            LootTables::EquipmentTrialChamber { inner } => inner,
            LootTables::EquipmentTrialChamberRanged { inner } => inner,
            LootTables::EquipmentTrialChamberMelee { inner } => inner,
            LootTables::ShipwreckMap { inner } => inner,
            LootTables::ShipwreckSupply { inner } => inner,
            LootTables::ShipwreckTreasure { inner } => inner,
            LootTables::SimpleDungeon { inner } => inner,
            LootTables::SpawnBonusChest { inner } => inner,
            LootTables::StrongholdCorridor { inner } => inner,
            LootTables::StrongholdCrossing { inner } => inner,
            LootTables::StrongholdLibrary { inner } => inner,
            LootTables::UnderwaterRuinBig { inner } => inner,
            LootTables::UnderwaterRuinSmall { inner } => inner,
            LootTables::VillageArmorer { inner } => inner,
            LootTables::VillageButcher { inner } => inner,
            LootTables::VillageCartographer { inner } => inner,
            LootTables::VillageDesertHouse { inner } => inner,
            LootTables::VillageFisher { inner } => inner,
            LootTables::VillageFletcher { inner } => inner,
            LootTables::VillageMason { inner } => inner,
            LootTables::VillagePlainsHouse { inner } => inner,
            LootTables::VillageSavannaHouse { inner } => inner,
            LootTables::VillageShepherd { inner } => inner,
            LootTables::VillageSnowyHouse { inner } => inner,
            LootTables::VillageTaigaHouse { inner } => inner,
            LootTables::VillageTannery { inner } => inner,
            LootTables::VillageTemple { inner } => inner,
            LootTables::VillageToolsmith { inner } => inner,
            LootTables::VillageWeaponsmith { inner } => inner,
            LootTables::WoodlandMansion { inner } => inner,
            LootTables::ArmorStand { inner } => inner,
            LootTables::Axolotl { inner } => inner,
            LootTables::Bat { inner } => inner,
            LootTables::Bee { inner } => inner,
            LootTables::Blaze { inner } => inner,
            LootTables::Cat { inner } => inner,
            LootTables::CaveSpider { inner } => inner,
            LootTables::Chicken { inner } => inner,
            LootTables::Cod { inner } => inner,
            LootTables::Cow { inner } => inner,
            LootTables::Creeper { inner } => inner,
            LootTables::Dolphin { inner } => inner,
            LootTables::Donkey { inner } => inner,
            LootTables::Drowned { inner } => inner,
            LootTables::ElderGuardian { inner } => inner,
            LootTables::EnderDragon { inner } => inner,
            LootTables::Enderman { inner } => inner,
            LootTables::Endermite { inner } => inner,
            LootTables::Evoker { inner } => inner,
            LootTables::Fox { inner } => inner,
            LootTables::Ghast { inner } => inner,
            LootTables::Giant { inner } => inner,
            LootTables::GlowSquid { inner } => inner,
            LootTables::Goat { inner } => inner,
            LootTables::Guardian { inner } => inner,
            LootTables::Hoglin { inner } => inner,
            LootTables::Horse { inner } => inner,
            LootTables::Husk { inner } => inner,
            LootTables::Illusioner { inner } => inner,
            LootTables::IronGolem { inner } => inner,
            LootTables::Llama { inner } => inner,
            LootTables::MagmaCube { inner } => inner,
            LootTables::Mooshroom { inner } => inner,
            LootTables::Mule { inner } => inner,
            LootTables::Ocelot { inner } => inner,
            LootTables::Panda { inner } => inner,
            LootTables::Parrot { inner } => inner,
            LootTables::Phantom { inner } => inner,
            LootTables::Pig { inner } => inner,
            LootTables::Piglin { inner } => inner,
            LootTables::PiglinBrute { inner } => inner,
            LootTables::Pillager { inner } => inner,
            LootTables::Player { inner } => inner,
            LootTables::PolarBear { inner } => inner,
            LootTables::Pufferfish { inner } => inner,
            LootTables::Rabbit { inner } => inner,
            LootTables::Ravager { inner } => inner,
            LootTables::Salmon { inner } => inner,
            LootTables::Shulker { inner } => inner,
            LootTables::Silverfish { inner } => inner,
            LootTables::Skeleton { inner } => inner,
            LootTables::SkeletonHorse { inner } => inner,
            LootTables::Slime { inner } => inner,
            LootTables::SnowGolem { inner } => inner,
            LootTables::Spider { inner } => inner,
            LootTables::Squid { inner } => inner,
            LootTables::Stray { inner } => inner,
            LootTables::Strider { inner } => inner,
            LootTables::TraderLlama { inner } => inner,
            LootTables::TropicalFish { inner } => inner,
            LootTables::Turtle { inner } => inner,
            LootTables::Vex { inner } => inner,
            LootTables::Villager { inner } => inner,
            LootTables::Vindicator { inner } => inner,
            LootTables::WanderingTrader { inner } => inner,
            LootTables::Witch { inner } => inner,
            LootTables::Wither { inner } => inner,
            LootTables::WitherSkeleton { inner } => inner,
            LootTables::Wolf { inner } => inner,
            LootTables::Zoglin { inner } => inner,
            LootTables::Zombie { inner } => inner,
            LootTables::ZombieHorse { inner } => inner,
            LootTables::ZombieVillager { inner } => inner,
            LootTables::ZombifiedPiglin { inner } => inner,
            LootTables::ArmorerGift { inner } => inner,
            LootTables::ButcherGift { inner } => inner,
            LootTables::CartographerGift { inner } => inner,
            LootTables::CatMorningGift { inner } => inner,
            LootTables::ClericGift { inner } => inner,
            LootTables::FarmerGift { inner } => inner,
            LootTables::FishermanGift { inner } => inner,
            LootTables::Fishing { inner } => inner,
            LootTables::FishingFish { inner } => inner,
            LootTables::FishingJunk { inner } => inner,
            LootTables::FishingTreasure { inner } => inner,
            LootTables::FletcherGift { inner } => inner,
            LootTables::LeatherworkerGift { inner } => inner,
            LootTables::LibrarianGift { inner } => inner,
            LootTables::MasonGift { inner } => inner,
            LootTables::ShepherdGift { inner } => inner,
            LootTables::ToolsmithGift { inner } => inner,
            LootTables::WeaponsmithGift { inner } => inner,
            LootTables::SnifferDigging { inner } => inner,
            LootTables::PandaSneeze { inner } => inner,
            LootTables::PiglinBartering { inner } => inner,
            LootTables::TrialChamberKey { inner } => inner,
            LootTables::TrialChamberConsumables { inner } => inner,
            LootTables::OminousTrialChamberKey { inner } => inner,
            LootTables::OminousTrialChamberConsumables { inner } => inner,
            LootTables::TrialChamberItemsToDropWhenOminous { inner } => inner,
            LootTables::ShearingBogged { inner } => inner,
            LootTables::DesertWellArchaeology { inner } => inner,
            LootTables::DesertPyramidArchaeology { inner } => inner,
            LootTables::TrailRuinsArchaeologyCommon { inner } => inner,
            LootTables::TrailRuinsArchaeologyRare { inner } => inner,
            LootTables::OceanRuinWarmArchaeology { inner } => inner,
            LootTables::OceanRuinColdArchaeology { inner } => inner,
            LootTables::Sheep { inner } => inner,
            LootTables::SheepBlack { inner } => inner,
            LootTables::SheepBlue { inner } => inner,
            LootTables::SheepBrown { inner } => inner,
            LootTables::SheepCyan { inner } => inner,
            LootTables::SheepGray { inner } => inner,
            LootTables::SheepGreen { inner } => inner,
            LootTables::SheepLightBlue { inner } => inner,
            LootTables::SheepLightGray { inner } => inner,
            LootTables::SheepLime { inner } => inner,
            LootTables::SheepMagenta { inner } => inner,
            LootTables::SheepOrange { inner } => inner,
            LootTables::SheepPink { inner } => inner,
            LootTables::SheepPurple { inner } => inner,
            LootTables::SheepRed { inner } => inner,
            LootTables::SheepWhite { inner } => inner,
            LootTables::SheepYellow { inner } => inner,
        }
    }
}

impl<'mc> LootTables<'mc> {
    pub fn value_of(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        arg0: impl Into<String>,
    ) -> Result<LootTables<'mc>, Box<dyn std::error::Error>> {
        let val_1 = jni::objects::JObject::from(env.new_string(arg0.into())?);
        let cls = env.find_class("org/bukkit/loot/LootTables");
        let cls = env.translate_error_with_class(cls)?;
        let res = env.call_static_method(
            cls,
            "valueOf",
            "(Ljava/lang/String;)Lorg/bukkit/loot/LootTables;",
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = env.translate_error(res)?;
        let obj = res.l()?;
        let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
        let variant = env.translate_error(variant)?;
        let variant_str = env
            .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
            .to_string_lossy()
            .to_string();
        match variant_str.as_str() {
            "EMPTY" => Ok(LootTables::Empty {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ABANDONED_MINESHAFT" => Ok(LootTables::AbandonedMineshaft {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BURIED_TREASURE" => Ok(LootTables::BuriedTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_PYRAMID" => Ok(LootTables::DesertPyramid {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "END_CITY_TREASURE" => Ok(LootTables::EndCityTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "IGLOO_CHEST" => Ok(LootTables::IglooChest {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "JUNGLE_TEMPLE" => Ok(LootTables::JungleTemple {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "JUNGLE_TEMPLE_DISPENSER" => Ok(LootTables::JungleTempleDispenser {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "NETHER_BRIDGE" => Ok(LootTables::NetherBridge {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PILLAGER_OUTPOST" => Ok(LootTables::PillagerOutpost {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_TREASURE" => Ok(LootTables::BastionTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_OTHER" => Ok(LootTables::BastionOther {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_BRIDGE" => Ok(LootTables::BastionBridge {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BASTION_HOGLIN_STABLE" => Ok(LootTables::BastionHoglinStable {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ANCIENT_CITY" => Ok(LootTables::AncientCity {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ANCIENT_CITY_ICE_BOX" => Ok(LootTables::AncientCityIceBox {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "RUINED_PORTAL" => Ok(LootTables::RuinedPortal {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_REWARD" => Ok(LootTables::TrialChambersReward {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_REWARD_COMMON" => Ok(LootTables::TrialChambersRewardCommon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_REWARD_RARE" => Ok(LootTables::TrialChambersRewardRare {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_REWARD_UNIQUE" => Ok(LootTables::TrialChambersRewardUnique {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_REWARD_OMINOUS" => Ok(LootTables::TrialChambersRewardOminous {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_REWARD_OMINOUS_COMMON" => {
                Ok(LootTables::TrialChambersRewardOminousCommon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                })
            }
            "TRIAL_CHAMBERS_REWARD_OMINOUS_RARE" => {
                Ok(LootTables::TrialChambersRewardOminousRare {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                })
            }
            "TRIAL_CHAMBERS_REWARD_OMINOUS_UNIQUE" => {
                Ok(LootTables::TrialChambersRewardOminousUnique {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                })
            }
            "TRIAL_CHAMBERS_SUPPLY" => Ok(LootTables::TrialChambersSupply {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_CORRIDOR" => Ok(LootTables::TrialChambersCorridor {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_INTERSECTION" => Ok(LootTables::TrialChambersIntersection {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_INTERSECTION_BARREL" => {
                Ok(LootTables::TrialChambersIntersectionBarrel {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                })
            }
            "TRIAL_CHAMBERS_ENTRANCE" => Ok(LootTables::TrialChambersEntrance {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_CORRIDOR_DISPENSER" => Ok(LootTables::TrialChambersCorridorDispenser {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_CHAMBER_DISPENSER" => Ok(LootTables::TrialChambersChamberDispenser {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_WATER_DISPENSER" => Ok(LootTables::TrialChambersWaterDispenser {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBERS_CORRIDOR_POT" => Ok(LootTables::TrialChambersCorridorPot {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "EQUIPMENT_TRIAL_CHAMBER" => Ok(LootTables::EquipmentTrialChamber {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "EQUIPMENT_TRIAL_CHAMBER_RANGED" => Ok(LootTables::EquipmentTrialChamberRanged {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "EQUIPMENT_TRIAL_CHAMBER_MELEE" => Ok(LootTables::EquipmentTrialChamberMelee {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_MAP" => Ok(LootTables::ShipwreckMap {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_SUPPLY" => Ok(LootTables::ShipwreckSupply {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHIPWRECK_TREASURE" => Ok(LootTables::ShipwreckTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SIMPLE_DUNGEON" => Ok(LootTables::SimpleDungeon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SPAWN_BONUS_CHEST" => Ok(LootTables::SpawnBonusChest {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_CORRIDOR" => Ok(LootTables::StrongholdCorridor {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_CROSSING" => Ok(LootTables::StrongholdCrossing {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRONGHOLD_LIBRARY" => Ok(LootTables::StrongholdLibrary {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "UNDERWATER_RUIN_BIG" => Ok(LootTables::UnderwaterRuinBig {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "UNDERWATER_RUIN_SMALL" => Ok(LootTables::UnderwaterRuinSmall {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_ARMORER" => Ok(LootTables::VillageArmorer {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_BUTCHER" => Ok(LootTables::VillageButcher {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_CARTOGRAPHER" => Ok(LootTables::VillageCartographer {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_DESERT_HOUSE" => Ok(LootTables::VillageDesertHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_FISHER" => Ok(LootTables::VillageFisher {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_FLETCHER" => Ok(LootTables::VillageFletcher {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_MASON" => Ok(LootTables::VillageMason {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_PLAINS_HOUSE" => Ok(LootTables::VillagePlainsHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SAVANNA_HOUSE" => Ok(LootTables::VillageSavannaHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SHEPHERD" => Ok(LootTables::VillageShepherd {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_SNOWY_HOUSE" => Ok(LootTables::VillageSnowyHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TAIGA_HOUSE" => Ok(LootTables::VillageTaigaHouse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TANNERY" => Ok(LootTables::VillageTannery {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TEMPLE" => Ok(LootTables::VillageTemple {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_TOOLSMITH" => Ok(LootTables::VillageToolsmith {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGE_WEAPONSMITH" => Ok(LootTables::VillageWeaponsmith {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WOODLAND_MANSION" => Ok(LootTables::WoodlandMansion {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ARMOR_STAND" => Ok(LootTables::ArmorStand {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "AXOLOTL" => Ok(LootTables::Axolotl {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BAT" => Ok(LootTables::Bat {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BEE" => Ok(LootTables::Bee {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BLAZE" => Ok(LootTables::Blaze {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CAT" => Ok(LootTables::Cat {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CAVE_SPIDER" => Ok(LootTables::CaveSpider {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CHICKEN" => Ok(LootTables::Chicken {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "COD" => Ok(LootTables::Cod {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "COW" => Ok(LootTables::Cow {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CREEPER" => Ok(LootTables::Creeper {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DOLPHIN" => Ok(LootTables::Dolphin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DONKEY" => Ok(LootTables::Donkey {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DROWNED" => Ok(LootTables::Drowned {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ELDER_GUARDIAN" => Ok(LootTables::ElderGuardian {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDER_DRAGON" => Ok(LootTables::EnderDragon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDERMAN" => Ok(LootTables::Enderman {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ENDERMITE" => Ok(LootTables::Endermite {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "EVOKER" => Ok(LootTables::Evoker {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FOX" => Ok(LootTables::Fox {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GHAST" => Ok(LootTables::Ghast {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GIANT" => Ok(LootTables::Giant {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GLOW_SQUID" => Ok(LootTables::GlowSquid {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GOAT" => Ok(LootTables::Goat {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "GUARDIAN" => Ok(LootTables::Guardian {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "HOGLIN" => Ok(LootTables::Hoglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "HORSE" => Ok(LootTables::Horse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "HUSK" => Ok(LootTables::Husk {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ILLUSIONER" => Ok(LootTables::Illusioner {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "IRON_GOLEM" => Ok(LootTables::IronGolem {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "LLAMA" => Ok(LootTables::Llama {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MAGMA_CUBE" => Ok(LootTables::MagmaCube {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MOOSHROOM" => Ok(LootTables::Mooshroom {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MULE" => Ok(LootTables::Mule {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OCELOT" => Ok(LootTables::Ocelot {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PANDA" => Ok(LootTables::Panda {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PARROT" => Ok(LootTables::Parrot {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PHANTOM" => Ok(LootTables::Phantom {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIG" => Ok(LootTables::Pig {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN" => Ok(LootTables::Piglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_BRUTE" => Ok(LootTables::PiglinBrute {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PILLAGER" => Ok(LootTables::Pillager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PLAYER" => Ok(LootTables::Player {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "POLAR_BEAR" => Ok(LootTables::PolarBear {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PUFFERFISH" => Ok(LootTables::Pufferfish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "RABBIT" => Ok(LootTables::Rabbit {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "RAVAGER" => Ok(LootTables::Ravager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SALMON" => Ok(LootTables::Salmon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHULKER" => Ok(LootTables::Shulker {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SILVERFISH" => Ok(LootTables::Silverfish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SKELETON" => Ok(LootTables::Skeleton {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SKELETON_HORSE" => Ok(LootTables::SkeletonHorse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SLIME" => Ok(LootTables::Slime {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SNOW_GOLEM" => Ok(LootTables::SnowGolem {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SPIDER" => Ok(LootTables::Spider {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SQUID" => Ok(LootTables::Squid {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRAY" => Ok(LootTables::Stray {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "STRIDER" => Ok(LootTables::Strider {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRADER_LLAMA" => Ok(LootTables::TraderLlama {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TROPICAL_FISH" => Ok(LootTables::TropicalFish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TURTLE" => Ok(LootTables::Turtle {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VEX" => Ok(LootTables::Vex {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VILLAGER" => Ok(LootTables::Villager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "VINDICATOR" => Ok(LootTables::Vindicator {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WANDERING_TRADER" => Ok(LootTables::WanderingTrader {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WITCH" => Ok(LootTables::Witch {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WITHER" => Ok(LootTables::Wither {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WITHER_SKELETON" => Ok(LootTables::WitherSkeleton {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WOLF" => Ok(LootTables::Wolf {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOGLIN" => Ok(LootTables::Zoglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE" => Ok(LootTables::Zombie {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_HORSE" => Ok(LootTables::ZombieHorse {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIE_VILLAGER" => Ok(LootTables::ZombieVillager {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ZOMBIFIED_PIGLIN" => Ok(LootTables::ZombifiedPiglin {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "ARMORER_GIFT" => Ok(LootTables::ArmorerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "BUTCHER_GIFT" => Ok(LootTables::ButcherGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CARTOGRAPHER_GIFT" => Ok(LootTables::CartographerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CAT_MORNING_GIFT" => Ok(LootTables::CatMorningGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "CLERIC_GIFT" => Ok(LootTables::ClericGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FARMER_GIFT" => Ok(LootTables::FarmerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHERMAN_GIFT" => Ok(LootTables::FishermanGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING" => Ok(LootTables::Fishing {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_FISH" => Ok(LootTables::FishingFish {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_JUNK" => Ok(LootTables::FishingJunk {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FISHING_TREASURE" => Ok(LootTables::FishingTreasure {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "FLETCHER_GIFT" => Ok(LootTables::FletcherGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "LEATHERWORKER_GIFT" => Ok(LootTables::LeatherworkerGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "LIBRARIAN_GIFT" => Ok(LootTables::LibrarianGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "MASON_GIFT" => Ok(LootTables::MasonGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEPHERD_GIFT" => Ok(LootTables::ShepherdGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TOOLSMITH_GIFT" => Ok(LootTables::ToolsmithGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "WEAPONSMITH_GIFT" => Ok(LootTables::WeaponsmithGift {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SNIFFER_DIGGING" => Ok(LootTables::SnifferDigging {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PANDA_SNEEZE" => Ok(LootTables::PandaSneeze {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "PIGLIN_BARTERING" => Ok(LootTables::PiglinBartering {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBER_KEY" => Ok(LootTables::TrialChamberKey {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBER_CONSUMABLES" => Ok(LootTables::TrialChamberConsumables {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OMINOUS_TRIAL_CHAMBER_KEY" => Ok(LootTables::OminousTrialChamberKey {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OMINOUS_TRIAL_CHAMBER_CONSUMABLES" => Ok(LootTables::OminousTrialChamberConsumables {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRIAL_CHAMBER_ITEMS_TO_DROP_WHEN_OMINOUS" => {
                Ok(LootTables::TrialChamberItemsToDropWhenOminous {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                })
            }
            "SHEARING_BOGGED" => Ok(LootTables::ShearingBogged {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_WELL_ARCHAEOLOGY" => Ok(LootTables::DesertWellArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "DESERT_PYRAMID_ARCHAEOLOGY" => Ok(LootTables::DesertPyramidArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => Ok(LootTables::TrailRuinsArchaeologyCommon {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "TRAIL_RUINS_ARCHAEOLOGY_RARE" => Ok(LootTables::TrailRuinsArchaeologyRare {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OCEAN_RUIN_WARM_ARCHAEOLOGY" => Ok(LootTables::OceanRuinWarmArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "OCEAN_RUIN_COLD_ARCHAEOLOGY" => Ok(LootTables::OceanRuinColdArchaeology {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP" => Ok(LootTables::Sheep {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BLACK" => Ok(LootTables::SheepBlack {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BLUE" => Ok(LootTables::SheepBlue {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_BROWN" => Ok(LootTables::SheepBrown {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_CYAN" => Ok(LootTables::SheepCyan {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_GRAY" => Ok(LootTables::SheepGray {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_GREEN" => Ok(LootTables::SheepGreen {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIGHT_BLUE" => Ok(LootTables::SheepLightBlue {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIGHT_GRAY" => Ok(LootTables::SheepLightGray {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_LIME" => Ok(LootTables::SheepLime {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_MAGENTA" => Ok(LootTables::SheepMagenta {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_ORANGE" => Ok(LootTables::SheepOrange {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_PINK" => Ok(LootTables::SheepPink {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_PURPLE" => Ok(LootTables::SheepPurple {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_RED" => Ok(LootTables::SheepRed {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_WHITE" => Ok(LootTables::SheepWhite {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),
            "SHEEP_YELLOW" => Ok(LootTables::SheepYellow {
                inner: LootTablesStruct::from_raw(env, obj)?,
            }),

            _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
        }
    }
}

#[repr(C)]
pub struct LootTablesStruct<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootTables<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        match self {
            Self::Empty { inner } => inner.0.clone(),
            Self::AbandonedMineshaft { inner } => inner.0.clone(),
            Self::BuriedTreasure { inner } => inner.0.clone(),
            Self::DesertPyramid { inner } => inner.0.clone(),
            Self::EndCityTreasure { inner } => inner.0.clone(),
            Self::IglooChest { inner } => inner.0.clone(),
            Self::JungleTemple { inner } => inner.0.clone(),
            Self::JungleTempleDispenser { inner } => inner.0.clone(),
            Self::NetherBridge { inner } => inner.0.clone(),
            Self::PillagerOutpost { inner } => inner.0.clone(),
            Self::BastionTreasure { inner } => inner.0.clone(),
            Self::BastionOther { inner } => inner.0.clone(),
            Self::BastionBridge { inner } => inner.0.clone(),
            Self::BastionHoglinStable { inner } => inner.0.clone(),
            Self::AncientCity { inner } => inner.0.clone(),
            Self::AncientCityIceBox { inner } => inner.0.clone(),
            Self::RuinedPortal { inner } => inner.0.clone(),
            Self::TrialChambersReward { inner } => inner.0.clone(),
            Self::TrialChambersRewardCommon { inner } => inner.0.clone(),
            Self::TrialChambersRewardRare { inner } => inner.0.clone(),
            Self::TrialChambersRewardUnique { inner } => inner.0.clone(),
            Self::TrialChambersRewardOminous { inner } => inner.0.clone(),
            Self::TrialChambersRewardOminousCommon { inner } => inner.0.clone(),
            Self::TrialChambersRewardOminousRare { inner } => inner.0.clone(),
            Self::TrialChambersRewardOminousUnique { inner } => inner.0.clone(),
            Self::TrialChambersSupply { inner } => inner.0.clone(),
            Self::TrialChambersCorridor { inner } => inner.0.clone(),
            Self::TrialChambersIntersection { inner } => inner.0.clone(),
            Self::TrialChambersIntersectionBarrel { inner } => inner.0.clone(),
            Self::TrialChambersEntrance { inner } => inner.0.clone(),
            Self::TrialChambersCorridorDispenser { inner } => inner.0.clone(),
            Self::TrialChambersChamberDispenser { inner } => inner.0.clone(),
            Self::TrialChambersWaterDispenser { inner } => inner.0.clone(),
            Self::TrialChambersCorridorPot { inner } => inner.0.clone(),
            Self::EquipmentTrialChamber { inner } => inner.0.clone(),
            Self::EquipmentTrialChamberRanged { inner } => inner.0.clone(),
            Self::EquipmentTrialChamberMelee { inner } => inner.0.clone(),
            Self::ShipwreckMap { inner } => inner.0.clone(),
            Self::ShipwreckSupply { inner } => inner.0.clone(),
            Self::ShipwreckTreasure { inner } => inner.0.clone(),
            Self::SimpleDungeon { inner } => inner.0.clone(),
            Self::SpawnBonusChest { inner } => inner.0.clone(),
            Self::StrongholdCorridor { inner } => inner.0.clone(),
            Self::StrongholdCrossing { inner } => inner.0.clone(),
            Self::StrongholdLibrary { inner } => inner.0.clone(),
            Self::UnderwaterRuinBig { inner } => inner.0.clone(),
            Self::UnderwaterRuinSmall { inner } => inner.0.clone(),
            Self::VillageArmorer { inner } => inner.0.clone(),
            Self::VillageButcher { inner } => inner.0.clone(),
            Self::VillageCartographer { inner } => inner.0.clone(),
            Self::VillageDesertHouse { inner } => inner.0.clone(),
            Self::VillageFisher { inner } => inner.0.clone(),
            Self::VillageFletcher { inner } => inner.0.clone(),
            Self::VillageMason { inner } => inner.0.clone(),
            Self::VillagePlainsHouse { inner } => inner.0.clone(),
            Self::VillageSavannaHouse { inner } => inner.0.clone(),
            Self::VillageShepherd { inner } => inner.0.clone(),
            Self::VillageSnowyHouse { inner } => inner.0.clone(),
            Self::VillageTaigaHouse { inner } => inner.0.clone(),
            Self::VillageTannery { inner } => inner.0.clone(),
            Self::VillageTemple { inner } => inner.0.clone(),
            Self::VillageToolsmith { inner } => inner.0.clone(),
            Self::VillageWeaponsmith { inner } => inner.0.clone(),
            Self::WoodlandMansion { inner } => inner.0.clone(),
            Self::ArmorStand { inner } => inner.0.clone(),
            Self::Axolotl { inner } => inner.0.clone(),
            Self::Bat { inner } => inner.0.clone(),
            Self::Bee { inner } => inner.0.clone(),
            Self::Blaze { inner } => inner.0.clone(),
            Self::Cat { inner } => inner.0.clone(),
            Self::CaveSpider { inner } => inner.0.clone(),
            Self::Chicken { inner } => inner.0.clone(),
            Self::Cod { inner } => inner.0.clone(),
            Self::Cow { inner } => inner.0.clone(),
            Self::Creeper { inner } => inner.0.clone(),
            Self::Dolphin { inner } => inner.0.clone(),
            Self::Donkey { inner } => inner.0.clone(),
            Self::Drowned { inner } => inner.0.clone(),
            Self::ElderGuardian { inner } => inner.0.clone(),
            Self::EnderDragon { inner } => inner.0.clone(),
            Self::Enderman { inner } => inner.0.clone(),
            Self::Endermite { inner } => inner.0.clone(),
            Self::Evoker { inner } => inner.0.clone(),
            Self::Fox { inner } => inner.0.clone(),
            Self::Ghast { inner } => inner.0.clone(),
            Self::Giant { inner } => inner.0.clone(),
            Self::GlowSquid { inner } => inner.0.clone(),
            Self::Goat { inner } => inner.0.clone(),
            Self::Guardian { inner } => inner.0.clone(),
            Self::Hoglin { inner } => inner.0.clone(),
            Self::Horse { inner } => inner.0.clone(),
            Self::Husk { inner } => inner.0.clone(),
            Self::Illusioner { inner } => inner.0.clone(),
            Self::IronGolem { inner } => inner.0.clone(),
            Self::Llama { inner } => inner.0.clone(),
            Self::MagmaCube { inner } => inner.0.clone(),
            Self::Mooshroom { inner } => inner.0.clone(),
            Self::Mule { inner } => inner.0.clone(),
            Self::Ocelot { inner } => inner.0.clone(),
            Self::Panda { inner } => inner.0.clone(),
            Self::Parrot { inner } => inner.0.clone(),
            Self::Phantom { inner } => inner.0.clone(),
            Self::Pig { inner } => inner.0.clone(),
            Self::Piglin { inner } => inner.0.clone(),
            Self::PiglinBrute { inner } => inner.0.clone(),
            Self::Pillager { inner } => inner.0.clone(),
            Self::Player { inner } => inner.0.clone(),
            Self::PolarBear { inner } => inner.0.clone(),
            Self::Pufferfish { inner } => inner.0.clone(),
            Self::Rabbit { inner } => inner.0.clone(),
            Self::Ravager { inner } => inner.0.clone(),
            Self::Salmon { inner } => inner.0.clone(),
            Self::Shulker { inner } => inner.0.clone(),
            Self::Silverfish { inner } => inner.0.clone(),
            Self::Skeleton { inner } => inner.0.clone(),
            Self::SkeletonHorse { inner } => inner.0.clone(),
            Self::Slime { inner } => inner.0.clone(),
            Self::SnowGolem { inner } => inner.0.clone(),
            Self::Spider { inner } => inner.0.clone(),
            Self::Squid { inner } => inner.0.clone(),
            Self::Stray { inner } => inner.0.clone(),
            Self::Strider { inner } => inner.0.clone(),
            Self::TraderLlama { inner } => inner.0.clone(),
            Self::TropicalFish { inner } => inner.0.clone(),
            Self::Turtle { inner } => inner.0.clone(),
            Self::Vex { inner } => inner.0.clone(),
            Self::Villager { inner } => inner.0.clone(),
            Self::Vindicator { inner } => inner.0.clone(),
            Self::WanderingTrader { inner } => inner.0.clone(),
            Self::Witch { inner } => inner.0.clone(),
            Self::Wither { inner } => inner.0.clone(),
            Self::WitherSkeleton { inner } => inner.0.clone(),
            Self::Wolf { inner } => inner.0.clone(),
            Self::Zoglin { inner } => inner.0.clone(),
            Self::Zombie { inner } => inner.0.clone(),
            Self::ZombieHorse { inner } => inner.0.clone(),
            Self::ZombieVillager { inner } => inner.0.clone(),
            Self::ZombifiedPiglin { inner } => inner.0.clone(),
            Self::ArmorerGift { inner } => inner.0.clone(),
            Self::ButcherGift { inner } => inner.0.clone(),
            Self::CartographerGift { inner } => inner.0.clone(),
            Self::CatMorningGift { inner } => inner.0.clone(),
            Self::ClericGift { inner } => inner.0.clone(),
            Self::FarmerGift { inner } => inner.0.clone(),
            Self::FishermanGift { inner } => inner.0.clone(),
            Self::Fishing { inner } => inner.0.clone(),
            Self::FishingFish { inner } => inner.0.clone(),
            Self::FishingJunk { inner } => inner.0.clone(),
            Self::FishingTreasure { inner } => inner.0.clone(),
            Self::FletcherGift { inner } => inner.0.clone(),
            Self::LeatherworkerGift { inner } => inner.0.clone(),
            Self::LibrarianGift { inner } => inner.0.clone(),
            Self::MasonGift { inner } => inner.0.clone(),
            Self::ShepherdGift { inner } => inner.0.clone(),
            Self::ToolsmithGift { inner } => inner.0.clone(),
            Self::WeaponsmithGift { inner } => inner.0.clone(),
            Self::SnifferDigging { inner } => inner.0.clone(),
            Self::PandaSneeze { inner } => inner.0.clone(),
            Self::PiglinBartering { inner } => inner.0.clone(),
            Self::TrialChamberKey { inner } => inner.0.clone(),
            Self::TrialChamberConsumables { inner } => inner.0.clone(),
            Self::OminousTrialChamberKey { inner } => inner.0.clone(),
            Self::OminousTrialChamberConsumables { inner } => inner.0.clone(),
            Self::TrialChamberItemsToDropWhenOminous { inner } => inner.0.clone(),
            Self::ShearingBogged { inner } => inner.0.clone(),
            Self::DesertWellArchaeology { inner } => inner.0.clone(),
            Self::DesertPyramidArchaeology { inner } => inner.0.clone(),
            Self::TrailRuinsArchaeologyCommon { inner } => inner.0.clone(),
            Self::TrailRuinsArchaeologyRare { inner } => inner.0.clone(),
            Self::OceanRuinWarmArchaeology { inner } => inner.0.clone(),
            Self::OceanRuinColdArchaeology { inner } => inner.0.clone(),
            Self::Sheep { inner } => inner.0.clone(),
            Self::SheepBlack { inner } => inner.0.clone(),
            Self::SheepBlue { inner } => inner.0.clone(),
            Self::SheepBrown { inner } => inner.0.clone(),
            Self::SheepCyan { inner } => inner.0.clone(),
            Self::SheepGray { inner } => inner.0.clone(),
            Self::SheepGreen { inner } => inner.0.clone(),
            Self::SheepLightBlue { inner } => inner.0.clone(),
            Self::SheepLightGray { inner } => inner.0.clone(),
            Self::SheepLime { inner } => inner.0.clone(),
            Self::SheepMagenta { inner } => inner.0.clone(),
            Self::SheepOrange { inner } => inner.0.clone(),
            Self::SheepPink { inner } => inner.0.clone(),
            Self::SheepPurple { inner } => inner.0.clone(),
            Self::SheepRed { inner } => inner.0.clone(),
            Self::SheepWhite { inner } => inner.0.clone(),
            Self::SheepYellow { inner } => inner.0.clone(),
        }
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        match self {
            Self::Empty { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::AbandonedMineshaft { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BuriedTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertPyramid { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EndCityTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IglooChest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JungleTemple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::JungleTempleDispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::NetherBridge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PillagerOutpost { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionOther { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionBridge { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::BastionHoglinStable { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AncientCity { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::AncientCityIceBox { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::RuinedPortal { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersReward { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersRewardCommon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersRewardRare { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersRewardUnique { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersRewardOminous { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersRewardOminousCommon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersRewardOminousRare { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersRewardOminousUnique { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersSupply { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersCorridor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersIntersection { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersIntersectionBarrel { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersEntrance { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersCorridorDispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersChamberDispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersWaterDispenser { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChambersCorridorPot { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EquipmentTrialChamber { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EquipmentTrialChamberRanged { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EquipmentTrialChamberMelee { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckMap { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckSupply { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShipwreckTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SimpleDungeon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SpawnBonusChest { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdCorridor { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdCrossing { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::StrongholdLibrary { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::UnderwaterRuinBig { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::UnderwaterRuinSmall { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageArmorer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageButcher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageCartographer { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageDesertHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageFisher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageFletcher { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageMason { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillagePlainsHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageSavannaHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageShepherd { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageSnowyHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTaigaHouse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTannery { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageTemple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageToolsmith { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::VillageWeaponsmith { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WoodlandMansion { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorStand { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Axolotl { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Bee { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Blaze { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::CaveSpider { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Chicken { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cod { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Cow { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Creeper { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Dolphin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Donkey { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Drowned { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ElderGuardian { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::EnderDragon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Enderman { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Endermite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Evoker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Fox { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ghast { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Giant { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::GlowSquid { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Goat { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Guardian { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Hoglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Horse { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Husk { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Illusioner { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::IronGolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Llama { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::MagmaCube { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mooshroom { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Mule { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ocelot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Panda { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Parrot { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Phantom { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Pig { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Piglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PiglinBrute { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Pillager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Player { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::PolarBear { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Pufferfish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Rabbit { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Ravager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Salmon { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Shulker { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Silverfish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Skeleton { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SkeletonHorse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Slime { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SnowGolem { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Spider { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Squid { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Stray { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Strider { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::TraderLlama { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TropicalFish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Turtle { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vex { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Villager { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Vindicator { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WanderingTrader { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Witch { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Wither { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::WitherSkeleton { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Wolf { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Zoglin { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::Zombie { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::ZombieHorse { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombieVillager { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ZombifiedPiglin { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ArmorerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ButcherGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CartographerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::CatMorningGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ClericGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FarmerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishermanGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Fishing { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::FishingFish { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishingJunk { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FishingTreasure { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::FletcherGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LeatherworkerGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::LibrarianGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::MasonGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShepherdGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ToolsmithGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::WeaponsmithGift { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SnifferDigging { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PandaSneeze { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::PiglinBartering { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChamberKey { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChamberConsumables { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OminousTrialChamberKey { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OminousTrialChamberConsumables { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrialChamberItemsToDropWhenOminous { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::ShearingBogged { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertWellArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::DesertPyramidArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrailRuinsArchaeologyCommon { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::TrailRuinsArchaeologyRare { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OceanRuinWarmArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::OceanRuinColdArchaeology { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::Sheep { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SheepBlack { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepBrown { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepCyan { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepGreen { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLightBlue { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLightGray { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepLime { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepMagenta { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepOrange { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepPink { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepPurple { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepRed { inner } => unsafe { jni::objects::JObject::from_raw(inner.1.clone()) },
            Self::SheepWhite { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
            Self::SheepYellow { inner } => unsafe {
                jni::objects::JObject::from_raw(inner.1.clone())
            },
        }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTables<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootTables from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTables")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTables object, got {}",
                name
            )
            .into())
        } else {
            let variant = env.call_method(&obj, "toString", "()Ljava/lang/String;", vec![]);
            let variant = env.translate_error(variant)?;
            let variant_str = env
                .get_string(unsafe { &jni::objects::JString::from_raw(variant.as_jni().l) })?
                .to_string_lossy()
                .to_string();
            match variant_str.as_str() {
                "EMPTY" => Ok(LootTables::Empty {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ABANDONED_MINESHAFT" => Ok(LootTables::AbandonedMineshaft {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BURIED_TREASURE" => Ok(LootTables::BuriedTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_PYRAMID" => Ok(LootTables::DesertPyramid {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "END_CITY_TREASURE" => Ok(LootTables::EndCityTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "IGLOO_CHEST" => Ok(LootTables::IglooChest {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "JUNGLE_TEMPLE" => Ok(LootTables::JungleTemple {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "JUNGLE_TEMPLE_DISPENSER" => Ok(LootTables::JungleTempleDispenser {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "NETHER_BRIDGE" => Ok(LootTables::NetherBridge {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PILLAGER_OUTPOST" => Ok(LootTables::PillagerOutpost {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_TREASURE" => Ok(LootTables::BastionTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_OTHER" => Ok(LootTables::BastionOther {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_BRIDGE" => Ok(LootTables::BastionBridge {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BASTION_HOGLIN_STABLE" => Ok(LootTables::BastionHoglinStable {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ANCIENT_CITY" => Ok(LootTables::AncientCity {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ANCIENT_CITY_ICE_BOX" => Ok(LootTables::AncientCityIceBox {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "RUINED_PORTAL" => Ok(LootTables::RuinedPortal {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_REWARD" => Ok(LootTables::TrialChambersReward {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_REWARD_COMMON" => Ok(LootTables::TrialChambersRewardCommon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_REWARD_RARE" => Ok(LootTables::TrialChambersRewardRare {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_REWARD_UNIQUE" => Ok(LootTables::TrialChambersRewardUnique {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_REWARD_OMINOUS" => Ok(LootTables::TrialChambersRewardOminous {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_REWARD_OMINOUS_COMMON" => {
                    Ok(LootTables::TrialChambersRewardOminousCommon {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRIAL_CHAMBERS_REWARD_OMINOUS_RARE" => {
                    Ok(LootTables::TrialChambersRewardOminousRare {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRIAL_CHAMBERS_REWARD_OMINOUS_UNIQUE" => {
                    Ok(LootTables::TrialChambersRewardOminousUnique {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRIAL_CHAMBERS_SUPPLY" => Ok(LootTables::TrialChambersSupply {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_CORRIDOR" => Ok(LootTables::TrialChambersCorridor {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_INTERSECTION" => Ok(LootTables::TrialChambersIntersection {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_INTERSECTION_BARREL" => {
                    Ok(LootTables::TrialChambersIntersectionBarrel {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRIAL_CHAMBERS_ENTRANCE" => Ok(LootTables::TrialChambersEntrance {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_CORRIDOR_DISPENSER" => {
                    Ok(LootTables::TrialChambersCorridorDispenser {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRIAL_CHAMBERS_CHAMBER_DISPENSER" => {
                    Ok(LootTables::TrialChambersChamberDispenser {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRIAL_CHAMBERS_WATER_DISPENSER" => Ok(LootTables::TrialChambersWaterDispenser {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBERS_CORRIDOR_POT" => Ok(LootTables::TrialChambersCorridorPot {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "EQUIPMENT_TRIAL_CHAMBER" => Ok(LootTables::EquipmentTrialChamber {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "EQUIPMENT_TRIAL_CHAMBER_RANGED" => Ok(LootTables::EquipmentTrialChamberRanged {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "EQUIPMENT_TRIAL_CHAMBER_MELEE" => Ok(LootTables::EquipmentTrialChamberMelee {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_MAP" => Ok(LootTables::ShipwreckMap {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_SUPPLY" => Ok(LootTables::ShipwreckSupply {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHIPWRECK_TREASURE" => Ok(LootTables::ShipwreckTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SIMPLE_DUNGEON" => Ok(LootTables::SimpleDungeon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SPAWN_BONUS_CHEST" => Ok(LootTables::SpawnBonusChest {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_CORRIDOR" => Ok(LootTables::StrongholdCorridor {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_CROSSING" => Ok(LootTables::StrongholdCrossing {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRONGHOLD_LIBRARY" => Ok(LootTables::StrongholdLibrary {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "UNDERWATER_RUIN_BIG" => Ok(LootTables::UnderwaterRuinBig {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "UNDERWATER_RUIN_SMALL" => Ok(LootTables::UnderwaterRuinSmall {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_ARMORER" => Ok(LootTables::VillageArmorer {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_BUTCHER" => Ok(LootTables::VillageButcher {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_CARTOGRAPHER" => Ok(LootTables::VillageCartographer {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_DESERT_HOUSE" => Ok(LootTables::VillageDesertHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_FISHER" => Ok(LootTables::VillageFisher {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_FLETCHER" => Ok(LootTables::VillageFletcher {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_MASON" => Ok(LootTables::VillageMason {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_PLAINS_HOUSE" => Ok(LootTables::VillagePlainsHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SAVANNA_HOUSE" => Ok(LootTables::VillageSavannaHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SHEPHERD" => Ok(LootTables::VillageShepherd {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_SNOWY_HOUSE" => Ok(LootTables::VillageSnowyHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TAIGA_HOUSE" => Ok(LootTables::VillageTaigaHouse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TANNERY" => Ok(LootTables::VillageTannery {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TEMPLE" => Ok(LootTables::VillageTemple {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_TOOLSMITH" => Ok(LootTables::VillageToolsmith {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGE_WEAPONSMITH" => Ok(LootTables::VillageWeaponsmith {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WOODLAND_MANSION" => Ok(LootTables::WoodlandMansion {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ARMOR_STAND" => Ok(LootTables::ArmorStand {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "AXOLOTL" => Ok(LootTables::Axolotl {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BAT" => Ok(LootTables::Bat {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BEE" => Ok(LootTables::Bee {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BLAZE" => Ok(LootTables::Blaze {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CAT" => Ok(LootTables::Cat {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CAVE_SPIDER" => Ok(LootTables::CaveSpider {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CHICKEN" => Ok(LootTables::Chicken {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "COD" => Ok(LootTables::Cod {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "COW" => Ok(LootTables::Cow {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CREEPER" => Ok(LootTables::Creeper {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DOLPHIN" => Ok(LootTables::Dolphin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DONKEY" => Ok(LootTables::Donkey {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DROWNED" => Ok(LootTables::Drowned {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ELDER_GUARDIAN" => Ok(LootTables::ElderGuardian {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDER_DRAGON" => Ok(LootTables::EnderDragon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDERMAN" => Ok(LootTables::Enderman {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ENDERMITE" => Ok(LootTables::Endermite {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "EVOKER" => Ok(LootTables::Evoker {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FOX" => Ok(LootTables::Fox {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GHAST" => Ok(LootTables::Ghast {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GIANT" => Ok(LootTables::Giant {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GLOW_SQUID" => Ok(LootTables::GlowSquid {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GOAT" => Ok(LootTables::Goat {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "GUARDIAN" => Ok(LootTables::Guardian {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "HOGLIN" => Ok(LootTables::Hoglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "HORSE" => Ok(LootTables::Horse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "HUSK" => Ok(LootTables::Husk {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ILLUSIONER" => Ok(LootTables::Illusioner {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "IRON_GOLEM" => Ok(LootTables::IronGolem {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "LLAMA" => Ok(LootTables::Llama {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MAGMA_CUBE" => Ok(LootTables::MagmaCube {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MOOSHROOM" => Ok(LootTables::Mooshroom {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MULE" => Ok(LootTables::Mule {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OCELOT" => Ok(LootTables::Ocelot {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PANDA" => Ok(LootTables::Panda {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PARROT" => Ok(LootTables::Parrot {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PHANTOM" => Ok(LootTables::Phantom {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIG" => Ok(LootTables::Pig {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN" => Ok(LootTables::Piglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_BRUTE" => Ok(LootTables::PiglinBrute {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PILLAGER" => Ok(LootTables::Pillager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PLAYER" => Ok(LootTables::Player {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "POLAR_BEAR" => Ok(LootTables::PolarBear {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PUFFERFISH" => Ok(LootTables::Pufferfish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "RABBIT" => Ok(LootTables::Rabbit {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "RAVAGER" => Ok(LootTables::Ravager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SALMON" => Ok(LootTables::Salmon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHULKER" => Ok(LootTables::Shulker {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SILVERFISH" => Ok(LootTables::Silverfish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SKELETON" => Ok(LootTables::Skeleton {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SKELETON_HORSE" => Ok(LootTables::SkeletonHorse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SLIME" => Ok(LootTables::Slime {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SNOW_GOLEM" => Ok(LootTables::SnowGolem {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SPIDER" => Ok(LootTables::Spider {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SQUID" => Ok(LootTables::Squid {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRAY" => Ok(LootTables::Stray {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "STRIDER" => Ok(LootTables::Strider {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRADER_LLAMA" => Ok(LootTables::TraderLlama {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TROPICAL_FISH" => Ok(LootTables::TropicalFish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TURTLE" => Ok(LootTables::Turtle {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VEX" => Ok(LootTables::Vex {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VILLAGER" => Ok(LootTables::Villager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "VINDICATOR" => Ok(LootTables::Vindicator {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WANDERING_TRADER" => Ok(LootTables::WanderingTrader {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WITCH" => Ok(LootTables::Witch {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WITHER" => Ok(LootTables::Wither {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WITHER_SKELETON" => Ok(LootTables::WitherSkeleton {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WOLF" => Ok(LootTables::Wolf {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOGLIN" => Ok(LootTables::Zoglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE" => Ok(LootTables::Zombie {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_HORSE" => Ok(LootTables::ZombieHorse {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIE_VILLAGER" => Ok(LootTables::ZombieVillager {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ZOMBIFIED_PIGLIN" => Ok(LootTables::ZombifiedPiglin {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "ARMORER_GIFT" => Ok(LootTables::ArmorerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "BUTCHER_GIFT" => Ok(LootTables::ButcherGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CARTOGRAPHER_GIFT" => Ok(LootTables::CartographerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CAT_MORNING_GIFT" => Ok(LootTables::CatMorningGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "CLERIC_GIFT" => Ok(LootTables::ClericGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FARMER_GIFT" => Ok(LootTables::FarmerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHERMAN_GIFT" => Ok(LootTables::FishermanGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING" => Ok(LootTables::Fishing {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_FISH" => Ok(LootTables::FishingFish {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_JUNK" => Ok(LootTables::FishingJunk {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FISHING_TREASURE" => Ok(LootTables::FishingTreasure {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "FLETCHER_GIFT" => Ok(LootTables::FletcherGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "LEATHERWORKER_GIFT" => Ok(LootTables::LeatherworkerGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "LIBRARIAN_GIFT" => Ok(LootTables::LibrarianGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "MASON_GIFT" => Ok(LootTables::MasonGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEPHERD_GIFT" => Ok(LootTables::ShepherdGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TOOLSMITH_GIFT" => Ok(LootTables::ToolsmithGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "WEAPONSMITH_GIFT" => Ok(LootTables::WeaponsmithGift {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SNIFFER_DIGGING" => Ok(LootTables::SnifferDigging {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PANDA_SNEEZE" => Ok(LootTables::PandaSneeze {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "PIGLIN_BARTERING" => Ok(LootTables::PiglinBartering {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBER_KEY" => Ok(LootTables::TrialChamberKey {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRIAL_CHAMBER_CONSUMABLES" => Ok(LootTables::TrialChamberConsumables {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OMINOUS_TRIAL_CHAMBER_KEY" => Ok(LootTables::OminousTrialChamberKey {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OMINOUS_TRIAL_CHAMBER_CONSUMABLES" => {
                    Ok(LootTables::OminousTrialChamberConsumables {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "TRIAL_CHAMBER_ITEMS_TO_DROP_WHEN_OMINOUS" => {
                    Ok(LootTables::TrialChamberItemsToDropWhenOminous {
                        inner: LootTablesStruct::from_raw(env, obj)?,
                    })
                }
                "SHEARING_BOGGED" => Ok(LootTables::ShearingBogged {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_WELL_ARCHAEOLOGY" => Ok(LootTables::DesertWellArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "DESERT_PYRAMID_ARCHAEOLOGY" => Ok(LootTables::DesertPyramidArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRAIL_RUINS_ARCHAEOLOGY_COMMON" => Ok(LootTables::TrailRuinsArchaeologyCommon {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "TRAIL_RUINS_ARCHAEOLOGY_RARE" => Ok(LootTables::TrailRuinsArchaeologyRare {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OCEAN_RUIN_WARM_ARCHAEOLOGY" => Ok(LootTables::OceanRuinWarmArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "OCEAN_RUIN_COLD_ARCHAEOLOGY" => Ok(LootTables::OceanRuinColdArchaeology {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP" => Ok(LootTables::Sheep {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BLACK" => Ok(LootTables::SheepBlack {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BLUE" => Ok(LootTables::SheepBlue {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_BROWN" => Ok(LootTables::SheepBrown {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_CYAN" => Ok(LootTables::SheepCyan {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_GRAY" => Ok(LootTables::SheepGray {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_GREEN" => Ok(LootTables::SheepGreen {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIGHT_BLUE" => Ok(LootTables::SheepLightBlue {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIGHT_GRAY" => Ok(LootTables::SheepLightGray {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_LIME" => Ok(LootTables::SheepLime {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_MAGENTA" => Ok(LootTables::SheepMagenta {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_ORANGE" => Ok(LootTables::SheepOrange {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_PINK" => Ok(LootTables::SheepPink {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_PURPLE" => Ok(LootTables::SheepPurple {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_RED" => Ok(LootTables::SheepRed {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_WHITE" => Ok(LootTables::SheepWhite {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                "SHEEP_YELLOW" => Ok(LootTables::SheepYellow {
                    inner: LootTablesStruct::from_raw(env, obj)?,
                }),
                _ => Err(eyre::eyre!("String gaven for variant was invalid").into()),
            }
        }
    }
}

impl<'mc> JNIRaw<'mc> for LootTablesStruct<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTablesStruct<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LootTablesStruct from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTables")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTablesStruct object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LootTablesStruct<'mc> {
    pub fn values(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
    ) -> Result<crate::loot::LootTables<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTables;");
        let cls = jni.find_class("org/bukkit/loot/LootTables");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.call_static_method(cls, "values", sig.as_str(), vec![]);
        let res = jni.translate_error(res)?;
        let obj = res.l()?;
        crate::loot::LootTables::from_raw(&jni, obj)
    }

    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Get the {@link LootTable} corresponding to this constant. This is
    /// equivalent to calling {@code Bukkit.getLootTable(this.getKey());}.
    pub fn loot_table(&self) -> Result<crate::loot::LootTable<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootTable::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct LootTable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootTable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootTable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootTable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootTable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootTable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LootTable<'mc> {
    /// Returns a mutable list of loot generated by this LootTable.
    pub fn populate_loot(
        &self,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        context: impl Into<crate::loot::LootContext<'mc>>,
    ) -> Result<Vec<crate::inventory::ItemStack<'mc>>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Ljava/util/Random;Lorg/bukkit/loot/LootContext;)Ljava/util/Collection;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "populateLoot",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
            ],
        );
        let res = self.jni_ref().translate_error(res)?;
        let mut new_vec = Vec::new();
        let col = blackboxmc_java::util::JavaCollection::from_raw(&self.jni_ref(), res.l()?)?;
        let iter = col.iterator()?;
        while iter.has_next()? {
            let obj = iter.next()?;
            new_vec.push(crate::inventory::ItemStack::from_raw(&self.jni_ref(), obj)?);
        }
        Ok(new_vec)
    }
    /// Attempt to fill an inventory with this LootTable's loot.
    pub fn fill_inventory(
        &self,
        inventory: impl Into<crate::inventory::Inventory<'mc>>,
        random: impl Into<blackboxmc_java::util::JavaRandom<'mc>>,
        context: impl Into<crate::loot::LootContext<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from(
            "(Lorg/bukkit/inventory/Inventory;Ljava/util/Random;Lorg/bukkit/loot/LootContext;)V",
        );
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(inventory.into().jni_object().clone())
        });
        let val_2 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(random.into().jni_object().clone())
        });
        let val_3 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(context.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "fillInventory",
            sig.as_str(),
            vec![
                jni::objects::JValueGen::from(val_1),
                jni::objects::JValueGen::from(val_2),
                jni::objects::JValueGen::from(val_3),
            ],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Return the namespaced identifier for this object.
    pub fn key(&self) -> Result<crate::NamespacedKey<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/NamespacedKey;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKey", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::NamespacedKey::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
impl<'mc> Into<crate::Keyed<'mc>> for LootTable<'mc> {
    fn into(self) -> crate::Keyed<'mc> {
        crate::Keyed::from_raw(&self.jni_ref(), self.1)
            .expect("Error converting LootTable into crate::Keyed")
    }
}
#[repr(C)]
pub struct Lootable<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for Lootable<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for Lootable<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate Lootable from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/Lootable")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a Lootable object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> Lootable<'mc> {
    /// Set the loot table for a container or entity.
    ///
    /// To remove a loot table use null. Do not use {@link LootTables#EMPTY} to
    /// clear a LootTable.
    pub fn set_loot_table(
        &self,
        table: impl Into<crate::loot::LootTable<'mc>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/loot/LootTable;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(table.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setLootTable",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Gets the Loot Table attached to this block or entity.
    ///
    /// If an block/entity does not have a loot table, this will return null, NOT
    /// an empty loot table.
    pub fn loot_table(
        &self,
    ) -> Result<Option<crate::loot::LootTable<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootTable;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootTable", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::loot::LootTable::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Set the seed used when this Loot Table generates loot.
    pub fn set_seed(&self, seed: i64) -> Result<(), Box<dyn std::error::Error>> {
        let sig = String::from("(J)V");
        let val_1 = jni::objects::JValueGen::Long(seed);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "setSeed",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        self.jni_ref().translate_error(res)?;
        Ok(())
    }
    /// Get the Loot Table's seed.
    ///
    /// The seed is used when generating loot.
    pub fn seed(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let sig = String::from("()J");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getSeed", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.j()?)
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct LootContext<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootContext<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootContext<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(eyre::eyre!("Tried to instantiate LootContext from null object.").into());
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootContext")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootContext object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LootContext<'mc> {
    /// The {@link Location} to store where the loot will be generated.
    pub fn location(&self) -> Result<crate::Location<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/Location;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLocation", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::Location::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Represents the {@link org.bukkit.potion.PotionEffectType#LUCK} that an
    /// entity can have. The higher the value the better chance of receiving more
    /// loot.
    pub fn luck(&self) -> Result<f32, Box<dyn std::error::Error>> {
        let sig = String::from("()F");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getLuck", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.f()?)
    }
    #[deprecated]
    /// Represents the {@link org.bukkit.enchantments.Enchantment#LOOTING} the {@link #getKiller()} entity has on their equipped item. This value is only set via {@link LootContext.Builder#lootingModifier(int)}. If not set, the {@link #getKiller()} entity's looting level will be used instead.
    pub fn looting_modifier(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let sig = String::from("()I");
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "getLootingModifier",
            sig.as_str(),
            vec![],
        );
        let res = self.jni_ref().translate_error(res)?;
        Ok(res.i()?)
    }
    /// Get the {@link Entity} that was killed. Can be null.
    pub fn looted_entity(
        &self,
    ) -> Result<Option<crate::entity::Entity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/Entity;");
        let res =
            self.jni_ref()
                .call_method(&self.jni_object(), "getLootedEntity", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::Entity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }
    /// Get the {@link HumanEntity} who killed the {@link #getLootedEntity()}.
    /// Can be null.
    pub fn killer(
        &self,
    ) -> Result<Option<crate::entity::HumanEntity<'mc>>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/entity/HumanEntity;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "getKiller", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        if unsafe { jni::objects::JObject::from_raw(res.as_jni().l) }.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::entity::HumanEntity::from_raw(
            &self.jni_ref(),
            unsafe { jni::objects::JObject::from_raw(res.l()?.clone()) },
        )?))
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
#[repr(C)]
pub struct LootContextBuilder<'mc>(
    pub(crate) blackboxmc_general::SharedJNIEnv<'mc>,
    pub(crate) jni::objects::JObject<'mc>,
);

impl<'mc> JNIRaw<'mc> for LootContextBuilder<'mc> {
    fn jni_ref(&self) -> blackboxmc_general::SharedJNIEnv<'mc> {
        self.0.clone()
    }
    fn jni_object(&self) -> jni::objects::JObject<'mc> {
        unsafe { jni::objects::JObject::from_raw(self.1.clone()) }
    }
}
impl<'mc> JNIInstantiatable<'mc> for LootContextBuilder<'mc> {
    fn from_raw(
        env: &blackboxmc_general::SharedJNIEnv<'mc>,
        obj: jni::objects::JObject<'mc>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if obj.is_null() {
            return Err(
                eyre::eyre!("Tried to instantiate LootContextBuilder from null object.").into(),
            );
        }
        let (valid, name) = env.validate_name(&obj, "org/bukkit/loot/LootContext/Builder")?;
        if !valid {
            Err(eyre::eyre!(
                "Invalid argument passed. Expected a LootContextBuilder object, got {}",
                name
            )
            .into())
        } else {
            Ok(Self(env.clone(), obj))
        }
    }
}

impl<'mc> LootContextBuilder<'mc> {
    /// Creates a new LootContext.Builder instance to facilitate easy
    /// creation of {@link LootContext}s.
    pub fn new(
        jni: &blackboxmc_general::SharedJNIEnv<'mc>,
        location: impl Into<crate::Location<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/Location;)V");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(location.into().jni_object().clone())
        });
        let cls = jni.find_class("org/bukkit/loot/LootContext/Builder");
        let cls = jni.translate_error_with_class(cls)?;
        let res = jni.new_object(
            cls,
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = jni.translate_error_no_gen(res)?;
        crate::loot::LootContextBuilder::from_raw(&jni, res)
    }
    /// Set how much luck to have when generating loot.
    pub fn luck(
        &self,
        luck: f32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(F)Lorg/bukkit/loot/LootContext/Builder;");
        let val_1 = jni::objects::JValueGen::Float(luck);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "luck",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    #[deprecated]
    /// Set the {@link org.bukkit.enchantments.Enchantment#LOOTING} level equivalent to use when generating loot. Values less than or equal to 0 will force the {@link LootTable} to only return a single {@link org.bukkit.inventory.ItemStack} per pool.
    pub fn looting_modifier(
        &self,
        modifier: i32,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(I)Lorg/bukkit/loot/LootContext/Builder;");
        let val_1 = jni::objects::JValueGen::Int(modifier);
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootingModifier",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// The entity that was killed.
    pub fn looted_entity(
        &self,
        looted_entity: impl Into<crate::entity::Entity<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("(Lorg/bukkit/entity/Entity;)Lorg/bukkit/loot/LootContext/Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(looted_entity.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "lootedEntity",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Set the {@link org.bukkit.entity.HumanEntity} that killed
    /// {@link #getLootedEntity()}. This entity will be used to get the
    /// looting level if {@link #lootingModifier(int)} is not set.
    pub fn killer(
        &self,
        killer: impl Into<crate::entity::HumanEntity<'mc>>,
    ) -> Result<crate::loot::LootContextBuilder<'mc>, Box<dyn std::error::Error>> {
        let sig =
            String::from("(Lorg/bukkit/entity/HumanEntity;)Lorg/bukkit/loot/LootContext/Builder;");
        let val_1 = jni::objects::JValueGen::Object(unsafe {
            jni::objects::JObject::from_raw(killer.into().jni_object().clone())
        });
        let res = self.jni_ref().call_method(
            &self.jni_object(),
            "killer",
            sig.as_str(),
            vec![jni::objects::JValueGen::from(val_1)],
        );
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContextBuilder::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }
    /// Create a new {@link LootContext} instance using the supplied
    /// parameters.
    pub fn build(&self) -> Result<crate::loot::LootContext<'mc>, Box<dyn std::error::Error>> {
        let sig = String::from("()Lorg/bukkit/loot/LootContext;");
        let res = self
            .jni_ref()
            .call_method(&self.jni_object(), "build", sig.as_str(), vec![]);
        let res = self.jni_ref().translate_error(res)?;
        crate::loot::LootContext::from_raw(&self.jni_ref(), unsafe {
            jni::objects::JObject::from_raw(res.l()?.clone())
        })
    }

    pub fn instance_of(&self, other: impl Into<String>) -> Result<bool, jni::errors::Error> {
        let cls = &self.jni_ref().find_class(other.into().as_str())?;
        self.jni_ref().is_instance_of(&self.jni_object(), cls)
    }
}
