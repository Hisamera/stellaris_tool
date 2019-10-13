use std::fs;
use std::error::Error;

pub struct Folder {
    name: String,
    subfolders: Option<Vec<Folder>>,
}

// Cannot be done on the stack until this issue (https://github.com/rust-lang/rust/issues/44580) is resolved
// Because Rust still hasn't implemented const generics in future something like this
// struct RectangularArray<T, const WIDTH: usize, const HEIGHT: usize> {
//    array: [[T; WIDTH]; HEIGHT],
// }
// Should be possible, until then it HAS to be in Vectors even though we know size of an array 
// at compile time
// If someone is able to remedy this, so it can live on stack, I'd glady incorporate this here.
// Well now I know the size of it, but it'll change when I add functionalty, won't it?
pub fn populate_folder_structure<'main> (folder_structure: &mut std::vec::Vec<Folder>){
    folder_structure.push(
        Folder{name: "common".to_owned(), subfolders: 
            Some(
                vec![
                    Folder {name: "agendas".to_owned(), subfolders: None},
                    Folder {name: "ai_budget".to_owned(), subfolders: None},
                    Folder {name: "ambient_objects".to_owned(), subfolders: None},
                    Folder {name: "anomalies".to_owned(), subfolders: None},
                    Folder {name: "armies".to_owned(), subfolders: None},
                    Folder {name: "ascension_perks".to_owned(), subfolders: None},
                    Folder {name: "asteroid_belts".to_owned(), subfolders: None},
                    Folder {name: "attitudes".to_owned(), subfolders: None},
                    Folder {name: "bombardment_stances".to_owned(), subfolders: None},
                    Folder {name: "buildings".to_owned(), subfolders: None},
                    Folder {name: "button_effects".to_owned(), subfolders: None},
                    Folder {name: "bypass".to_owned(), subfolders: None},
                    Folder {name: "casus_belli".to_owned(), subfolders: None},
                    Folder {name: "colony_types".to_owned(), subfolders: None},
                    Folder {name: "colors".to_owned(), subfolders: None},
                    Folder {name: "component_sets".to_owned(), subfolders: None},
                    Folder {name: "component_tags".to_owned(), subfolders: None},
                    Folder {name: "component_templates".to_owned(), subfolders: None},
                    Folder {name: "country_customization".to_owned(), subfolders: None},
                    Folder {name: "country_types".to_owned(), subfolders: None},
                    Folder {name: "decisions".to_owned(), subfolders: None},
                    Folder {name: "defines".to_owned(), subfolders: None},
                    Folder {name: "deposits".to_owned(), subfolders: None},
                    Folder {name: "deposit_categories".to_owned(), subfolders: None},
                    Folder {name: "diplomacy_economy".to_owned(), subfolders: None},
                    Folder {name: "diplomatic_actions".to_owned(), subfolders: None},
                    Folder {name: "diplo_phrases".to_owned(), subfolders: None},
                    Folder {name: "districts".to_owned(), subfolders: None},
                    Folder {name: "economic_categories".to_owned(), subfolders: None},
                    Folder {name: "edicts".to_owned(), subfolders: None},
                    Folder {name: "ethics".to_owned(), subfolders: None},
                    Folder {name: "event_chains".to_owned(), subfolders: None},
                    Folder {name: "fallen_empires".to_owned(), subfolders: None},
                    Folder {name: "game_rules".to_owned(), subfolders: None},
                    Folder {name: "global_ship_designs".to_owned(), subfolders: None},
                    Folder {name: "graphical_culture".to_owned(), subfolders: None},
                    Folder {name: "leader_classes".to_owned(), subfolders: None},
                    Folder {name: "mandates".to_owned(), subfolders: None},
                    Folder {name: "map_modes".to_owned(), subfolders: None},
                    Folder {name: "megastructures".to_owned(), subfolders: None},
                    Folder {name: "name_lists".to_owned(), subfolders: None},
                    Folder {name: "notification_modifiers".to_owned(), subfolders: None},
                    Folder {name: "observation_station_missions".to_owned(), subfolders: None},
                    Folder {name: "on_actions".to_owned(), subfolders: None},
                    Folder {name: "opinion_modifiers".to_owned(), subfolders: None},
                    Folder {name: "personalities".to_owned(), subfolders: None},
                    Folder {name: "planet_classes".to_owned(), subfolders: None},
                    Folder {name: "planet_modifiers".to_owned(), subfolders: None},
                    Folder {name: "policies".to_owned(), subfolders: None},
                    Folder {name: "pop_categories".to_owned(), subfolders: None},
                    Folder {name: "pop_faction_types".to_owned(), subfolders: None},
                    Folder {name: "pop_jobs".to_owned(), subfolders: None},
                    Folder {name: "precursor_civilizations".to_owned(), subfolders: None},
                    Folder {name: "scripted_effects".to_owned(), subfolders: None},
                    Folder {name: "scripted_loc".to_owned(), subfolders: None},
                    Folder {name: "scripted_triggers".to_owned(), subfolders: None},
                    Folder {name: "scripted_variables".to_owned(), subfolders: None},
                    Folder {name: "section_templates".to_owned(), subfolders: None},
                    Folder {name: "sector_focuses".to_owned(), subfolders: None},
                    Folder {name: "sector_types".to_owned(), subfolders: None},
                    Folder {name: "ship_behaviors".to_owned(), subfolders: None},
                    Folder {name: "ship_sizes".to_owned(), subfolders: None},
                    Folder {name: "solar_system_initializers".to_owned(), subfolders: None},
                    Folder {name: "special_projects".to_owned(), subfolders: None},
                    Folder {name: "species_archetypes".to_owned(), subfolders: None},
                    Folder {name: "species_classes".to_owned(), subfolders: None},
                    Folder {name: "species_names".to_owned(), subfolders: None},
                    Folder {name: "species_rights".to_owned(), subfolders: None},
                    Folder {name: "starbase_buildings".to_owned(), subfolders: None},
                    Folder {name: "starbase_levels".to_owned(), subfolders: None},
                    Folder {name: "starbase_modules".to_owned(), subfolders: None},
                    Folder {name: "starbase_types".to_owned(), subfolders: None},
                    Folder {name: "start_screen_messages".to_owned(), subfolders: None},
                    Folder {name: "star_classes".to_owned(), subfolders: None},
                    Folder {name: "static_modifiers".to_owned(), subfolders: None},
                    Folder {name: "strategic_resources".to_owned(), subfolders: None},
                    Folder {name: "subjects".to_owned(), subfolders: None},
                    Folder {name: "system_types".to_owned(), subfolders: None},
                    Folder {name: "terraform".to_owned(), subfolders: None},
                    Folder {name: "trade_conversions".to_owned(), subfolders: None},
                    Folder {name: "traditions".to_owned(), subfolders: None},
                    Folder {name: "tradition_categories".to_owned(), subfolders: None},
                    Folder {name: "traits".to_owned(), subfolders: None},
                    Folder {name: "war_goals".to_owned(), subfolders: None},
                    Folder {name: "technology".to_owned(), subfolders: 
                        Some(
                            vec![
                                Folder {name: "category".to_owned(), subfolders: None},
                                Folder {name: "tier".to_owned(), subfolders: None},
                            ]
                        )
                    },
                    Folder {name: "random_names".to_owned(), subfolders: 
                    Some(
                            vec![
                                Folder {name: "base".to_owned(), subfolders: None},
                            ]
                        )
                    },
                    Folder {name: "governments".to_owned(), subfolders: 
                        Some(
                            vec![
                                Folder {name: "authorities".to_owned(), subfolders: None},
                                Folder {name: "civics".to_owned(), subfolders: None},
                            ]
                        )
                    },
                ]
            )
        }
    );
    folder_structure.push(
        Folder {name: "flags".to_owned(), subfolders: Some(
            vec![
                Folder {name: "backgrounds".to_owned(), subfolders: None},
                Folder {name: "blocky".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "corporate".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "domination".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "enclaves".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "human".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "ornate".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "paradox".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "pirate".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "pointy".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "special".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "spherical".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
                Folder {name: "zoological".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "map".to_owned(), subfolders: None}, 
                        Folder {name: "small".to_owned(), subfolders: None}, 
                    ]
                )},
            ]
        )}
    );
    folder_structure.push(
        Folder {name: "gfx".to_owned(), subfolders: Some(
            vec![
                Folder {name: "advisorwindow".to_owned(), subfolders: None},
                Folder {name: "arrows".to_owned(), subfolders: None},
                Folder {name: "cursors".to_owned(), subfolders: None},
                Folder {name: "event_pictures".to_owned(), subfolders: None},
                Folder {name: "fonts".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "arimo".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "FX".to_owned(), subfolders: None},
                Folder {name: "interface".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "anomaly".to_owned(), subfolders: None},
                        Folder {name: "archaeology".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "runes".to_owned(), subfolders: None},
                            ]
                        )},
                        Folder {name: "buttons".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "gamepad".to_owned(), subfolders: None},
                            ]
                        )},
                        Folder {name: "diplomacy".to_owned(), subfolders: None},
                        Folder {name: "elections".to_owned(), subfolders: None},
                        Folder {name: "event_window".to_owned(), subfolders: None},
                        Folder {name: "flags".to_owned(), subfolders: None},
                        Folder {name: "fleet_view".to_owned(), subfolders: None},
                        Folder {name: "frontend".to_owned(), subfolders: None},
                        Folder {name: "government_mod_window".to_owned(), subfolders: None},
                        Folder {name: "icons".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "achievements".to_owned(), subfolders: None},
                                Folder {name: "advisor_vo".to_owned(), subfolders: None},
                                Folder {name: "army_attachments".to_owned(), subfolders: None},
                                Folder {name: "ascension_perks".to_owned(), subfolders: None},
                                Folder {name: "buildings".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "old_production".to_owned(), subfolders: None},
                                        Folder {name: "unused".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "combat_stats".to_owned(), subfolders: None},
                                Folder {name: "controller".to_owned(), subfolders: None},
                                Folder {name: "decisions".to_owned(), subfolders: None},
                                Folder {name: "deposits".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "old".to_owned(), subfolders: None},
                                        Folder {name: "unused".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "diplomacy".to_owned(), subfolders: None},
                                Folder {name: "districts".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "grid_box".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "dlc".to_owned(), subfolders: None},
                                Folder {name: "ethics".to_owned(), subfolders: None},
                                Folder {name: "faction_icons".to_owned(), subfolders: None},
                                Folder {name: "governments".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "authorities".to_owned(), subfolders: None},
                                        Folder {name: "civics".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "jobs".to_owned(), subfolders: None},
                                Folder {name: "message".to_owned(), subfolders: None},
                                Folder {name: "modifiers".to_owned(), subfolders: None},
                                Folder {name: "modules".to_owned(), subfolders: None},
                                Folder {name: "planet_backgrounds".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "old".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "planet_modifiers".to_owned(), subfolders: None},
                                Folder {name: "pop_categories".to_owned(), subfolders: None},
                                Folder {name: "relics".to_owned(), subfolders: None},
                                Folder {name: "resources".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "old_unused".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "ship_parts".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "computers".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "ship_stats".to_owned(), subfolders: None},
                                Folder {name: "situation_log".to_owned(), subfolders: None},
                                Folder {name: "technologies".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "old_tech_icons".to_owned(), subfolders: None},
                                        Folder {name: "tech_templates".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "text_icons".to_owned(), subfolders: None},
                                Folder {name: "trade route icons".to_owned(), subfolders: None},
                                Folder {name: "trade_icons".to_owned(), subfolders: None},
                                Folder {name: "traditions".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "unused".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "traits".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "leader_traits".to_owned(), subfolders: None},
                                    ]
                                )},
                            ]
                        )},
                        Folder {name: "main".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "control_group".to_owned(), subfolders: None},
                            ]
                        )},
                        Folder {name: "market".to_owned(), subfolders: None},
                        Folder {name: "masks".to_owned(), subfolders: None},
                        Folder {name: "multiplayer".to_owned(), subfolders: None},
                        Folder {name: "musicplayer".to_owned(), subfolders: None},
                        Folder {name: "outliner".to_owned(), subfolders: None},
                        Folder {name: "planetview".to_owned(), subfolders: None},
                        Folder {name: "progressbars".to_owned(), subfolders: None},
                        Folder {name: "relics".to_owned(), subfolders: None},
                        Folder {name: "ship_designer".to_owned(), subfolders: None},
                        Folder {name: "situation_log".to_owned(), subfolders: None},
                        Folder {name: "sliders".to_owned(), subfolders: None},
                        Folder {name: "system".to_owned(), subfolders: None},
                        Folder {name: "tech_view".to_owned(), subfolders: None},
                        Folder {name: "tiles".to_owned(), subfolders: None},
                        Folder {name: "topbar".to_owned(), subfolders: None},
                        Folder {name: "traditions".to_owned(), subfolders: None},
                        Folder {name: "waroverview".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "keyicons".to_owned(), subfolders: None},
                Folder {name: "lights".to_owned(), subfolders: None},
                Folder {name: "loadingscreens".to_owned(), subfolders: None},
                Folder {name: "map".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "star_classes".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "models".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "add_ons".to_owned(), subfolders: None},
                        Folder {name: "combat_items".to_owned(), subfolders: None},
                        Folder {name: "galaxy_map".to_owned(), subfolders: None},
                        Folder {name: "planets".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "apocalypse_planet_effects".to_owned(), subfolders: None},
                                Folder {name: "distant_stars_planets".to_owned(), subfolders: None},
                            ]
                        )},
                        Folder {name: "portraits".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "advisor".to_owned(), subfolders: None},
                                Folder {name: "AI".to_owned(), subfolders: None},
                                Folder {name: "ancient_relics".to_owned(), subfolders: None},
                                Folder {name: "arthropoid".to_owned(), subfolders: None},
                                Folder {name: "avian".to_owned(), subfolders: None},
                                Folder {name: "distant_stars".to_owned(), subfolders: None},
                                Folder {name: "extradimensional".to_owned(), subfolders: None},
                                Folder {name: "fungoid".to_owned(), subfolders: None},
                                Folder {name: "human".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "human_female_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "human_male_hair_update".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "humanoid".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "humanoid_02_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_05_female_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_05_male_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_05_female_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_hp_02_female_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_hp_02_male_beard_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_hp_11_female_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_hp_11_male_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_hp_12_female_hair_update".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_hp_12_male_hair_update".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "mammalian".to_owned(), subfolders: None},
                                Folder {name: "molluscoid".to_owned(), subfolders: None},
                                Folder {name: "plantoid".to_owned(), subfolders: None},
                                Folder {name: "portrait_items".to_owned(), subfolders: None},
                                Folder {name: "reptilian".to_owned(), subfolders: None},
                                Folder {name: "shroud".to_owned(), subfolders: None},
                                Folder {name: "swarm".to_owned(), subfolders: None},
                                Folder {name: "synthetic_dawn".to_owned(), subfolders: None},
                            ]
                        )},
                        Folder {name: "ships".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "ai_01".to_owned(), subfolders: None},
                                Folder {name: "arthropoid_01".to_owned(), subfolders: None},
                                Folder {name: "avian_01".to_owned(), subfolders: None},
                                Folder {name: "caravaneer_01".to_owned(), subfolders: None},
                                Folder {name: "colossus".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "arthropoid_01".to_owned(), subfolders: None},
                                        Folder {name: "avian_01".to_owned(), subfolders: None},
                                        Folder {name: "fallen_empire_01".to_owned(), subfolders: None},
                                        Folder {name: "fungoid_01".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_01".to_owned(), subfolders: None},
                                        Folder {name: "mammalian_01".to_owned(), subfolders: None},
                                        Folder {name: "molluscoid_01".to_owned(), subfolders: None},
                                        Folder {name: "plantoid_01".to_owned(), subfolders: None},
                                        Folder {name: "reptilian_01".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "extra_dimensional_01".to_owned(), subfolders: None},
                                Folder {name: "fallen_empire_01".to_owned(), subfolders: None},
                                Folder {name: "fallen_machine_empire_01".to_owned(), subfolders: None},
                                Folder {name: "fungoid_01".to_owned(), subfolders: None},
                                Folder {name: "gatebuilder_01".to_owned(), subfolders: None},
                                Folder {name: "guardians".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "ancient_station".to_owned(), subfolders: None},
                                        Folder {name: "dimensional_horror".to_owned(), subfolders: None},
                                        Folder {name: "dreadnought".to_owned(), subfolders: None},
                                        Folder {name: "drone_homebase".to_owned(), subfolders: None},
                                        Folder {name: "elder_tiyanki".to_owned(), subfolders: None},
                                        Folder {name: "hive_asteroids".to_owned(), subfolders: None},
                                        Folder {name: "pirate_galleon".to_owned(), subfolders: None},
                                        Folder {name: "scavenger_bot".to_owned(), subfolders: None},
                                        Folder {name: "space_dragon".to_owned(), subfolders: None},
                                        Folder {name: "stellarite".to_owned(), subfolders: None},
                                        Folder {name: "technosphere".to_owned(), subfolders: None},
                                        Folder {name: "voidspawn".to_owned(), subfolders: None},
                                        Folder {name: "wraith".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "humanoid_01".to_owned(), subfolders: None},
                                Folder {name: "mammalian_01".to_owned(), subfolders: None},
                                Folder {name: "megastructures".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "construction_platform".to_owned(), subfolders: None},
                                        Folder {name: "dyson_sphere".to_owned(), subfolders: None},
                                        Folder {name: "gateway".to_owned(), subfolders: None},
                                        Folder {name: "interstellar_assembly".to_owned(), subfolders: None},
                                        Folder {name: "matter_decompressor".to_owned(), subfolders: None},
                                        Folder {name: "mega_art_institution".to_owned(), subfolders: None},
                                        Folder {name: "spy_orb".to_owned(), subfolders: None},
                                        Folder {name: "strategic_coordination_center".to_owned(), subfolders: None},
                                        Folder {name: "think_tank".to_owned(), subfolders: None},
                                    ]
                                )},
                                Folder {name: "molluscoid_01".to_owned(), subfolders: None},
                                Folder {name: "other".to_owned(), subfolders: None},
                                Folder {name: "pirate_01".to_owned(), subfolders: None},
                                Folder {name: "plantoid_01".to_owned(), subfolders: None},
                                Folder {name: "reptilian_01".to_owned(), subfolders: None},
                                Folder {name: "shroud_01".to_owned(), subfolders: None},
                                Folder {name: "starbases".to_owned(), subfolders: None},
                                Folder {name: "swarm_01".to_owned(), subfolders: None},
                                Folder {name: "titans".to_owned(), subfolders: Some(
                                    vec![
                                        Folder {name: "arthropoid_01".to_owned(), subfolders: None},
                                        Folder {name: "avian_01".to_owned(), subfolders: None},
                                        Folder {name: "fungoid_01".to_owned(), subfolders: None},
                                        Folder {name: "humanoid_01".to_owned(), subfolders: None},
                                        Folder {name: "mammalian_01".to_owned(), subfolders: None},
                                        Folder {name: "molluscoid_01".to_owned(), subfolders: None},
                                        Folder {name: "plantoid_01".to_owned(), subfolders: None},
                                        Folder {name: "reptilian_01".to_owned(), subfolders: None},
                                    ]
                                )},
                            ]
                        )},
                        Folder {name: "system_map".to_owned(), subfolders: None},
                        Folder {name: "ui".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "particles".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "combat".to_owned(), subfolders: None},
                        Folder {name: "misc".to_owned(), subfolders: None},
                        Folder {name: "ships".to_owned(), subfolders: None},
                        Folder {name: "stars_and_planets".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "pingmap".to_owned(), subfolders: None},
                Folder {name: "portraits".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "asset_selectors".to_owned(), subfolders: None},
                        Folder {name: "city_sets".to_owned(), subfolders: None},
                        Folder {name: "environments".to_owned(), subfolders: None},
                        Folder {name: "leaders".to_owned(), subfolders: None},
                        Folder {name: "misc".to_owned(), subfolders: None},
                        Folder {name: "portraits".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "projectiles".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "planet_destruction".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "shipview".to_owned(), subfolders: None},
                Folder {name: "ugc".to_owned(), subfolders: None},
                Folder {name: "worldgfx".to_owned(), subfolders: None},
            ]
        )}
    );
    folder_structure.push(
        Folder {name: "interface".to_owned(), subfolders: Some(
            vec![
                Folder {name: "game_setup".to_owned(), subfolders: None},
                Folder {name: "pdx_online".to_owned(), subfolders: None},
                Folder {name: "portraits".to_owned(), subfolders: None},
                Folder {name: "resource_groups".to_owned(), subfolders: None},
            ]
        )}
    );
    folder_structure.push(
        Folder {name: "prescripted_countries".to_owned(), subfolders: None},
    );
    folder_structure.push(
        Folder {name: "sound".to_owned(), subfolders: Some(
            vec![
                Folder {name: "advisor_voice_types".to_owned(), subfolders: None},
                Folder {name: "ambient".to_owned(), subfolders: None},
                Folder {name: "ancient_relics".to_owned(), subfolders: None},
                Folder {name: "apocalypse".to_owned(), subfolders: None},
                Folder {name: "audiofiles".to_owned(), subfolders: None},
                Folder {name: "distant_stars".to_owned(), subfolders: None},
                Folder {name: "event".to_owned(), subfolders: None},
                Folder {name: "guardians".to_owned(), subfolders: None},
                Folder {name: "gui".to_owned(), subfolders: None},
                Folder {name: "humanoids".to_owned(), subfolders: None},
                Folder {name: "megacorp".to_owned(), subfolders: None},
                Folder {name: "other_entities".to_owned(), subfolders: None},
                Folder {name: "placeholders".to_owned(), subfolders: None},
                Folder {name: "portrait".to_owned(), subfolders: None},
                Folder {name: "ship_designer".to_owned(), subfolders: None},
                Folder {name: "synthetic_dawn".to_owned(), subfolders: None},
                Folder {name: "ui".to_owned(), subfolders: None},
                Folder {name: "units".to_owned(), subfolders: None},
                Folder {name: "utopia".to_owned(), subfolders: None},
                Folder {name: "vo".to_owned(), subfolders: Some(
                    vec![
                        Folder {name: "bajen".to_owned(), subfolders: None},
                        Folder {name: "ethics".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "authoritarian".to_owned(), subfolders: None},
                                Folder {name: "egalitarian".to_owned(), subfolders: None},
                                Folder {name: "hivemind".to_owned(), subfolders: None},
                                Folder {name: "machine_empire".to_owned(), subfolders: None},
                                Folder {name: "materialist".to_owned(), subfolders: None},
                                Folder {name: "militarist".to_owned(), subfolders: None},
                                Folder {name: "pacifist".to_owned(), subfolders: None},
                                Folder {name: "spiritualist".to_owned(), subfolders: None},
                                Folder {name: "xenophile".to_owned(), subfolders: None},
                                Folder {name: "xenophobe".to_owned(), subfolders: None},
                            ]
                        )},
                        Folder {name: "event".to_owned(), subfolders: None},
                        Folder {name: "generic".to_owned(), subfolders: None},
                        Folder {name: "notifications".to_owned(), subfolders: None},
                        Folder {name: "phenotypes".to_owned(), subfolders: Some(
                            vec![
                                Folder {name: "arthopoid".to_owned(), subfolders: None},
                                Folder {name: "avian".to_owned(), subfolders: None},
                                Folder {name: "fungoid".to_owned(), subfolders: None},
                                Folder {name: "human".to_owned(), subfolders: None},
                                Folder {name: "humanoid".to_owned(), subfolders: None},
                                Folder {name: "mammalian".to_owned(), subfolders: None},
                                Folder {name: "molluscoid".to_owned(), subfolders: None},
                                Folder {name: "reptilian".to_owned(), subfolders: None},
                            ]
                        )},
                        Folder {name: "respons".to_owned(), subfolders: None},
                        Folder {name: "tutorial".to_owned(), subfolders: None},
                    ]
                )},
                Folder {name: "weapons".to_owned(), subfolders: None},
            ]
        )},
    );
    folder_structure.push(
        Folder {name: "localisation_synced".to_owned(), subfolders: None}
    );
    folder_structure.push(
        Folder {name: "localisation".to_owned(), subfolders: Some(vec![
            Folder {name: "braz_por".to_owned(), subfolders: None},
            Folder {name: "english".to_owned(), subfolders: None},
            Folder {name: "french".to_owned(), subfolders: None},
            Folder {name: "german".to_owned(), subfolders: None},
            Folder {name: "polish".to_owned(), subfolders: None},
            Folder {name: "russian".to_owned(), subfolders: None},
            Folder {name: "simp_chinese".to_owned(), subfolders: None},
            Folder {name: "spanish".to_owned(), subfolders: None},
        ])}
    );
}

pub fn make_subfolders(parent_folder_name: &String, subfolder: &std::vec::Vec<Folder>, errors: &mut std::vec::Vec<String>){
    for next_level_folder in subfolder.iter() {
        let next_level_name = format!("{}\\{}", &parent_folder_name, next_level_folder.name);
        match fs::create_dir(&next_level_name) {
            Ok(()) => (),
            Err(err) => folder_error_handling(err, &next_level_name, errors),
        };
        match &next_level_folder.subfolders {
            None => continue,
            Some(subfolder) => make_subfolders(&next_level_name, subfolder,errors),
        }
    }
}

pub fn make_folder_structure(folder_structure: &std::vec::Vec<Folder>, root_dir: &String, errors: &mut std::vec::Vec<String>) {
    match fs::create_dir_all(&root_dir) {
        Ok(()) => (),
        Err(err) => folder_error_handling(err, &"Destination to root".to_owned(), errors)
    }
    for root_folder in folder_structure.iter() {
        let root_folder_name = format!("{}\\{}", root_dir, root_folder.name);
        match fs::create_dir(&root_folder_name) {
            Ok(()) => (),
            Err(err) => folder_error_handling(err, &"Destination to root".to_owned(), errors)
        }
        match &root_folder.subfolders {
            None => continue,
            Some(first_folder) => make_subfolders(&root_folder_name, &first_folder, errors),
        }
    }
}

fn folder_error_handling (file_error: std::io::Error, maybe_usefull: &String, errors: &mut std::vec::Vec<String>){
    if let Some(raw_os_err) = file_error.raw_os_error() {
        if raw_os_err == 80 || raw_os_err == 183 { 
            return 
        }
        return errors.push(format!("{} OS gave such error {} - {}", maybe_usefull, raw_os_err, file_error.description()))
    } else {
        return errors.push(format!("The error while creating folder {} came not from OS, unable to help.", maybe_usefull))
    }
    // panic!("The error while creating folder was not from OS which is super weird(Maybe usefull info: {}): {:?}", maybe_usefull, file_error)
}