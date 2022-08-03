use crate::{Effect, Unit};

pub fn effect(source: &Unit, target: &Unit) -> Effect {
    let src_hp = source.attr("hp");
    let tar_hp = target.attr("hp");
    let src_att = source.attr("att");

    Effect {
        dmg: true_zero_scale(src_hp, tar_hp, src_att),
    }
}

fn true_zero_scale(src: i32, tar: i32, dmg: i32) -> i32 {
    let ret = if tar > 0 {
        (src * dmg + tar / 2) / tar
    } else {
        0
    };

    if ret == 0 && dmg > 0 && tar > 0 {
        1
    } else {
        ret
    }
}
