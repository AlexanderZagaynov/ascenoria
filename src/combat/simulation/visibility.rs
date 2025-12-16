use super::super::types::Combatant;

pub fn update_visibility(
    attackers: &[Combatant],
    defenders: &[Combatant],
    positions_attackers: &[(i32, i32)],
    positions_defenders: &[(i32, i32)],
    attacker_visible: &mut [bool],
    defender_visible: &mut [bool],
) {
    for (i, vis) in attacker_visible.iter_mut().enumerate() {
        let viewer_pos = positions_attackers.get(i).copied().unwrap_or((0, 0));
        let scanner = attackers.get(i).map(|c| c.scanner_range).unwrap_or(0);
        *vis = can_detect(scanner, viewer_pos, defenders, positions_defenders);
    }

    for (i, vis) in defender_visible.iter_mut().enumerate() {
        let viewer_pos = positions_defenders.get(i).copied().unwrap_or((0, 0));
        let scanner = defenders.get(i).map(|c| c.scanner_range).unwrap_or(0);
        *vis = can_detect(scanner, viewer_pos, attackers, positions_attackers);
    }
}

fn can_detect(
    scanner_range: i32,
    viewer_pos: (i32, i32),
    targets: &[Combatant],
    target_positions: &[(i32, i32)],
) -> bool {
    for (idx, target) in targets.iter().enumerate() {
        if !target.alive() {
            continue;
        }
        let pos = target_positions.get(idx).copied().unwrap_or((0, 0));
        let dist_sq = distance_sq(viewer_pos, pos);
        if dist_sq <= (scanner_range as i64).pow(2) {
            return true;
        }
    }
    false
}

fn distance_sq(a: (i32, i32), b: (i32, i32)) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    dx * dx + dy * dy
}
