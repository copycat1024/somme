use crate::{Attr, Effect};

pub fn effect(source: &Attr, target: &Attr) -> Effect {
    let src_hp = source.get("hp");
    let tar_hp = target.get("hp");
    let src_att = source.get("att");

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
