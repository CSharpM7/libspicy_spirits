# Spicy Spirits
A plugin and api that allows modders to use smashline 2 features for spirit battles. Note that this **does not affect spirits themselves**, only their battles. Use the `plugin` folder as an example, the `api` folder won't be of interest to modders.

## How It Works
Spicy Spirits checks various information during the start of a match to make assumptions on if you are in a Spirit Battle or not. Hopefully each Spirit Battle has a unique combination of Rulesets and Enemies as to not repeat twice, but there's always a chance for false positives! To edit specific battles, you'll need to add their information to the api with `spicy_spirits::add_battle()` (make sure to only do this once)

## Example Plugin

An example can be found in this repo under the `plugin` [folder](https://github.com/CSharpM7/libspicy_spirits/tree/main/plugin/src). Feel free to use this as a template.

## How To Use
Install `libspicy_spirits` into your plugins directory. In your mod's workspace, make sure to add `spicy_spirits = { git = "https://github.com/CSharpM7/libspicy_spirits.git" }` to your `Cargo.toml`.

`spicy_spirits::add_battle()` is used to add a `SpiritBattle` to the list of Assigned Spirit Battles to look out for. A `SpiritBattle` uses these values:
```
let prog_enemies = vec![
    SpiritEnemy{
        kind: *FIGHTER_KIND_MARIO, //Fighter Kind
        color: 3 //Fighter Color. You can use -1 to accept any color (useful for Mob fighters)
    }
    //Because this is a vector, you can add more SpiritEnemy objects to this list. Note that if you have Allies in your Spirit Battle, you need to list them first before you list the enemies
];
{
battle_id: hash40("smoky_progg"), //Hash40 of the name of the spirit
battle_type: RULESET_STOCK, //The ruleset used. Can be _TIME,_STOCK, or _STAMINA
basic_init_hp: 0.0, //Starting HP during Stamina Mode. Use the first SpiritEnemy entry's value. Will be ignored during Stock/Time
basic_stock: 1, //Starting Stocks. Use the player's stock count.
stage_id: *StageID::Battle_Pikmin_Planet, //Stage ID
enemies: prog_enemies, //A list of all the enemies that appear at the start, the same vec variable definied above
};
```
NOTE: Do NOTE have `spicy_spirits::add_battle()` in a file/function that gets reloaded via the development plugin. See the example plugin on how to split a plugin into two (dev and devhook) so certain fuctions don't get reloaded.

During opff (or acmd or whatever else can be run after the initial countdown), you can use `spicy_spirits::get_sprit_battle_id()` to check if you are in one of the assigned spirit battle you added via `spicy_spirits::add_battle()`. In the example plugin, we check if `spicy_spirits::get_sprit_battle_id() == hash40("smoky_progg")`

Some other api functions of note are:
- `spicy_spirits::is_valid_game_mode()` checks if you are in a Spirits-related game mode. This includes Spirit Board, Adventure, and DLC Spirit Board.
- `spicy_spirits::is_ready()` checks if you are in a assigned spirit battle added via `add_battle()`, and that the countdown is finished
- `spicy_spirits::is_ready_init()` will only return True on the first frame after the countdown has finished, assuming you are in a assigned spirit battle
- `spicy_spirits::is_end()` will return true when the match ends, regardless of victory or defeat, with `spicy_spirits::is_end()_init` only being true on the first frame
- `get_sprit_battle_id()` gets the id of the battle you might be in. Will return 0 if you are not in an assigned spirit battle
