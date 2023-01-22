use role_playing_game::Player;

#[test]
fn test_1() {
    let dead_player = Player { health: 0, mana: None, level: 2 };
    let result = dead_player.revive();
    match result {
        Some(player) => {
            assert_eq!(player.health, 100);
            assert_eq!(player.mana, None);
            assert_eq!(player.level, 2);
        },
        None => assert!(false),
    }

    let alive_player = Player { health: 1, mana: Some(15), level: 11 };
    let result = alive_player.revive();
    match result {
        Some(_) => assert!(false),
        None => assert!(true),
    }

    let mut not_a_wizard_yet = Player { health: 79, mana: None, level: 9 };
    assert_eq!(not_a_wizard_yet.cast_spell(5), 0);
    assert_eq!(not_a_wizard_yet.health, 74);
    assert_eq!(not_a_wizard_yet.mana, None);

    let mut low_mana_wizard = Player { health: 93, mana: Some(3), level: 12 };
    assert_eq!(low_mana_wizard.cast_spell(10), 0);
    assert_eq!(low_mana_wizard.health, 93);
    assert_eq!(low_mana_wizard.mana, Some(3));

    let mut wizard = Player { health: 123, mana: Some(30), level: 18 };
    assert_eq!(wizard.cast_spell(10), 20);
    assert_eq!(wizard.health, 123);
    assert_eq!(wizard.mana, Some(20));
}

#[test]
fn test_reviving_dead_level10_player() {
    let dead_player = Player { health: 0, mana: None, level: 10 };
    let result = dead_player.revive();
    match result {
        Some(player) => {
            assert_eq!(player.health, 100);
            assert_eq!(player.mana.unwrap(), 100);
            assert_eq!(player.level, 10);
        },
        None => assert!(false),
    }
}

#[test]
fn test_cast_large_spell_with_no_mana_pool() {
    let mut wizard = Player { health: 10, mana: None, level: 5 };
    assert_eq!(wizard.cast_spell(100), 0);
    assert_eq!(wizard.health, 0);
    assert_eq!(wizard.mana, None);
}
