// AUTOGENERATED_START
#![allow(clippy::too_many_arguments)]
use crate::vanilla::{
    Attributes,
    AttributesEx1,
    AttributesEx2,
    AttributesEx3,
    AttributesEx4,
};

#[derive(Debug, Copy, Clone)]
pub struct Spell {
    entry: u16,
    school: i8,
    category: i16,
    cast_ui: i8,
    dispel: i8,
    mechanic: i8,
    attributes: Attributes,
    attributes_ex: AttributesEx1,
    attributes_ex2: AttributesEx2,
    attributes_ex3: AttributesEx3,
    attributes_ex4: AttributesEx4,
    stances: u32,
    stances_not: u32,
    targets: i32,
    target_creature_type: i16,
    requires_spell_focus: i16,
    casting_time_index: i16,
    recovery_time: i32,
    category_recovery_time: i32,
    interrupt_flags: i8,
    aura_interrupt_flags: i32,
    channel_interrupt_flags: i32,
    proc_flags: i32,
    proc_chance: i8,
    proc_charges: i8,
    max_level: i16,
    base_level: i8,
    spell_level: i8,
    duration_index: i16,
    power_type: u32,
    mana_cost: i32,
    mana_cost_per_level: i8,
    range_index: i16,
    speed: f32,
    modal_next_spell: i16,
    stack_amount: i16,
    equipped_item_class: i8,
    equipped_item_sub_class_mask: i32,
    equipped_item_inventory_type_mask: i32,
    spell_visual: i16,
    spell_icon_id: i16,
    active_icon_id: i16,
    spell_priority: i8,
    spell_name: &'static str,
    rank_text: &'static str,
    mana_cost_percentage: i16,
    start_recovery_category: i16,
    start_recovery_time: i16,
    spell_family_name: i8,
    spell_family_flags: i64,
    max_affected_targets: i8,
    dmg_class: i8,
    prevention_type: i8,
    stance_bar_order: i8,
    reagents: [Reagent; 8],
    effects: [SpellEffect; 3],
    totems: [Totem; 2],
}

impl Spell {
    pub const fn new(
        entry: u16,
        school: i8,
        category: i16,
        cast_ui: i8,
        dispel: i8,
        mechanic: i8,
        attributes: Attributes,
        attributes_ex: AttributesEx1,
        attributes_ex2: AttributesEx2,
        attributes_ex3: AttributesEx3,
        attributes_ex4: AttributesEx4,
        stances: u32,
        stances_not: u32,
        targets: i32,
        target_creature_type: i16,
        requires_spell_focus: i16,
        casting_time_index: i16,
        recovery_time: i32,
        category_recovery_time: i32,
        interrupt_flags: i8,
        aura_interrupt_flags: i32,
        channel_interrupt_flags: i32,
        proc_flags: i32,
        proc_chance: i8,
        proc_charges: i8,
        max_level: i16,
        base_level: i8,
        spell_level: i8,
        duration_index: i16,
        power_type: u32,
        mana_cost: i32,
        mana_cost_per_level: i8,
        range_index: i16,
        speed: f32,
        modal_next_spell: i16,
        stack_amount: i16,
        equipped_item_class: i8,
        equipped_item_sub_class_mask: i32,
        equipped_item_inventory_type_mask: i32,
        spell_visual: i16,
        spell_icon_id: i16,
        active_icon_id: i16,
        spell_priority: i8,
        spell_name: &'static str,
        rank_text: &'static str,
        mana_cost_percentage: i16,
        start_recovery_category: i16,
        start_recovery_time: i16,
        spell_family_name: i8,
        spell_family_flags: i64,
        max_affected_targets: i8,
        dmg_class: i8,
        prevention_type: i8,
        stance_bar_order: i8,
        reagent1: i32,
        reagent_count1: i32,
        reagent2: i32,
        reagent_count2: i32,
        reagent3: i32,
        reagent_count3: i32,
        reagent4: i32,
        reagent_count4: i32,
        reagent5: i32,
        reagent_count5: i32,
        reagent6: i32,
        reagent_count6: i32,
        reagent7: i32,
        reagent_count7: i32,
        reagent8: i32,
        reagent_count8: i32,
        effect1: i32,
        effect_die_sides1: i32,
        effect_base_dice1: i32,
        effect_dice_per_level1: f32,
        effect_real_points_per_level1: f32,
        effect_base_points1: i32,
        effect_mechanic1: i32,
        effect_implicit_target_a1: i32,
        effect_implicit_target_b1: i32,
        effect_radius_index1: i32,
        effect_apply_aura_name1: i32,
        effect_amplitude1: i32,
        effect_multiple_value1: f32,
        effect_chain_target1: i32,
        effect_item_type1: u32,
        effect_misc_value1: i32,
        effect_trigger_spell1: i32,
        effect_points_per_combo_point1: f32,
        dmg_multiplier1: f32,
        effect2: i32,
        effect_die_sides2: i32,
        effect_base_dice2: i32,
        effect_dice_per_level2: f32,
        effect_real_points_per_level2: f32,
        effect_base_points2: i32,
        effect_mechanic2: i32,
        effect_implicit_target_a2: i32,
        effect_implicit_target_b2: i32,
        effect_radius_index2: i32,
        effect_apply_aura_name2: i32,
        effect_amplitude2: i32,
        effect_multiple_value2: f32,
        effect_chain_target2: i32,
        effect_item_type2: u32,
        effect_misc_value2: i32,
        effect_trigger_spell2: i32,
        effect_points_per_combo_point2: f32,
        dmg_multiplier2: f32,
        effect3: i32,
        effect_die_sides3: i32,
        effect_base_dice3: i32,
        effect_dice_per_level3: f32,
        effect_real_points_per_level3: f32,
        effect_base_points3: i32,
        effect_mechanic3: i32,
        effect_implicit_target_a3: i32,
        effect_implicit_target_b3: i32,
        effect_radius_index3: i32,
        effect_apply_aura_name3: i32,
        effect_amplitude3: i32,
        effect_multiple_value3: f32,
        effect_chain_target3: i32,
        effect_item_type3: u32,
        effect_misc_value3: i32,
        effect_trigger_spell3: i32,
        effect_points_per_combo_point3: f32,
        dmg_multiplier3: f32,
        totem1: i32,
        totem2: i32,
    ) -> Self {
        Self {
            entry,
            school,
            category,
            cast_ui,
            dispel,
            mechanic,
            attributes,
            attributes_ex,
            attributes_ex2,
            attributes_ex3,
            attributes_ex4,
            stances,
            stances_not,
            targets,
            target_creature_type,
            requires_spell_focus,
            casting_time_index,
            recovery_time,
            category_recovery_time,
            interrupt_flags,
            aura_interrupt_flags,
            channel_interrupt_flags,
            proc_flags,
            proc_chance,
            proc_charges,
            max_level,
            base_level,
            spell_level,
            duration_index,
            power_type,
            mana_cost,
            mana_cost_per_level,
            range_index,
            speed,
            modal_next_spell,
            stack_amount,
            equipped_item_class,
            equipped_item_sub_class_mask,
            equipped_item_inventory_type_mask,
            spell_visual,
            spell_icon_id,
            active_icon_id,
            spell_priority,
            spell_name,
            rank_text,
            mana_cost_percentage,
            start_recovery_category,
            start_recovery_time,
            spell_family_name,
            spell_family_flags,
            max_affected_targets,
            dmg_class,
            prevention_type,
            stance_bar_order,
            reagents: [
            Reagent::new(
            reagent1,
            reagent_count1,
            ),
            Reagent::new(
            reagent2,
            reagent_count2,
            ),
            Reagent::new(
            reagent3,
            reagent_count3,
            ),
            Reagent::new(
            reagent4,
            reagent_count4,
            ),
            Reagent::new(
            reagent5,
            reagent_count5,
            ),
            Reagent::new(
            reagent6,
            reagent_count6,
            ),
            Reagent::new(
            reagent7,
            reagent_count7,
            ),
            Reagent::new(
            reagent8,
            reagent_count8,
            ),
            ],
            effects: [
            SpellEffect::new(
            effect1,
            effect_die_sides1,
            effect_base_dice1,
            effect_dice_per_level1,
            effect_real_points_per_level1,
            effect_base_points1,
            effect_mechanic1,
            effect_implicit_target_a1,
            effect_implicit_target_b1,
            effect_radius_index1,
            effect_apply_aura_name1,
            effect_amplitude1,
            effect_multiple_value1,
            effect_chain_target1,
            effect_item_type1,
            effect_misc_value1,
            effect_trigger_spell1,
            effect_points_per_combo_point1,
            dmg_multiplier1,
            ),
            SpellEffect::new(
            effect2,
            effect_die_sides2,
            effect_base_dice2,
            effect_dice_per_level2,
            effect_real_points_per_level2,
            effect_base_points2,
            effect_mechanic2,
            effect_implicit_target_a2,
            effect_implicit_target_b2,
            effect_radius_index2,
            effect_apply_aura_name2,
            effect_amplitude2,
            effect_multiple_value2,
            effect_chain_target2,
            effect_item_type2,
            effect_misc_value2,
            effect_trigger_spell2,
            effect_points_per_combo_point2,
            dmg_multiplier2,
            ),
            SpellEffect::new(
            effect3,
            effect_die_sides3,
            effect_base_dice3,
            effect_dice_per_level3,
            effect_real_points_per_level3,
            effect_base_points3,
            effect_mechanic3,
            effect_implicit_target_a3,
            effect_implicit_target_b3,
            effect_radius_index3,
            effect_apply_aura_name3,
            effect_amplitude3,
            effect_multiple_value3,
            effect_chain_target3,
            effect_item_type3,
            effect_misc_value3,
            effect_trigger_spell3,
            effect_points_per_combo_point3,
            dmg_multiplier3,
            ),
            ],
            totems: [
            Totem::new(
            totem1,
            ),
            Totem::new(
            totem2,
            ),
            ],
        }
    }
    pub const fn entry(&self) -> u32 {
        self.entry as u32
    }

    pub const fn school(&self) -> i32 {
        self.school as i32
    }

    pub const fn category(&self) -> i32 {
        self.category as i32
    }

    pub const fn cast_ui(&self) -> i32 {
        self.cast_ui as i32
    }

    pub const fn dispel(&self) -> i32 {
        self.dispel as i32
    }

    pub const fn mechanic(&self) -> i32 {
        self.mechanic as i32
    }

    pub const fn attributes(&self) -> Attributes {
        self.attributes
    }

    pub const fn attributes_ex(&self) -> AttributesEx1 {
        self.attributes_ex
    }

    pub const fn attributes_ex2(&self) -> AttributesEx2 {
        self.attributes_ex2
    }

    pub const fn attributes_ex3(&self) -> AttributesEx3 {
        self.attributes_ex3
    }

    pub const fn attributes_ex4(&self) -> AttributesEx4 {
        self.attributes_ex4
    }

    pub const fn stances(&self) -> u32 {
        self.stances
    }

    pub const fn stances_not(&self) -> u32 {
        self.stances_not
    }

    pub const fn targets(&self) -> i32 {
        self.targets
    }

    pub const fn target_creature_type(&self) -> i32 {
        self.target_creature_type as i32
    }

    pub const fn requires_spell_focus(&self) -> i32 {
        self.requires_spell_focus as i32
    }

    pub const fn caster_aura_state(&self) -> i32 {
        match self.entry {
            76 | 1495 | 6186 | 6568 | 6572 | 6574 | 7379 | 10374 | 11600 | 11601 | 12170 | 14251 | 14269 | 14270 | 14271 | 19130 | 25288 => 1,
            17170 => 2,
            26635 => 3,
            20271 | 27731 => 5,
            19306 | 20909 | 20910 => 7,
            _ => 0,
        }
    }

    pub const fn target_aura_state(&self) -> i32 {
        match self.entry {
            5308 | 6174 | 6175 | 7160 | 7938 | 16495 | 20539 | 20647 | 20658 | 20660 | 20661 | 20662 | 24239 | 24274 | 24275 | 31255 => 2,
            _ => 0,
        }
    }

    pub const fn casting_time_index(&self) -> i32 {
        self.casting_time_index as i32
    }

    pub const fn recovery_time(&self) -> i32 {
        self.recovery_time
    }

    pub const fn category_recovery_time(&self) -> i32 {
        self.category_recovery_time
    }

    pub const fn interrupt_flags(&self) -> i32 {
        self.interrupt_flags as i32
    }

    pub const fn aura_interrupt_flags(&self) -> i32 {
        self.aura_interrupt_flags
    }

    pub const fn channel_interrupt_flags(&self) -> i32 {
        self.channel_interrupt_flags
    }

    pub const fn proc_flags(&self) -> i32 {
        self.proc_flags
    }

    pub const fn proc_chance(&self) -> i32 {
        self.proc_chance as i32
    }

    pub const fn proc_charges(&self) -> i32 {
        self.proc_charges as i32
    }

    pub const fn max_level(&self) -> i32 {
        self.max_level as i32
    }

    pub const fn base_level(&self) -> i32 {
        self.base_level as i32
    }

    pub const fn spell_level(&self) -> i32 {
        self.spell_level as i32
    }

    pub const fn duration_index(&self) -> i32 {
        self.duration_index as i32
    }

    pub const fn power_type(&self) -> u32 {
        self.power_type
    }

    pub const fn mana_cost(&self) -> i32 {
        self.mana_cost
    }

    pub const fn mana_cost_per_level(&self) -> i32 {
        self.mana_cost_per_level as i32
    }

    pub const fn mana_per_second(&self) -> i32 {
        match self.entry {
            424 | 963 => 2,
            1116 | 3486 => 4,
            755 => 5,
            461 | 3698 => 10,
            3699 => 17,
            3700 => 24,
            10260 => 25,
            11693 => 33,
            11694 => 42,
            11695 => 52,
            16569 => 75,
            _ => 0,
        }
    }

    pub const fn mana_per_second_per_level(&self) -> i32 {
        0
    }

    pub const fn range_index(&self) -> i32 {
        self.range_index as i32
    }

    pub const fn speed(&self) -> f32 {
        self.speed
    }

    pub const fn modal_next_spell(&self) -> i32 {
        self.modal_next_spell as i32
    }

    pub const fn stack_amount(&self) -> i32 {
        self.stack_amount as i32
    }

    pub const fn equipped_item_class(&self) -> i32 {
        self.equipped_item_class as i32
    }

    pub const fn equipped_item_sub_class_mask(&self) -> i32 {
        self.equipped_item_sub_class_mask
    }

    pub const fn equipped_item_inventory_type_mask(&self) -> i32 {
        self.equipped_item_inventory_type_mask
    }

    pub const fn spell_visual(&self) -> i32 {
        self.spell_visual as i32
    }

    pub const fn spell_icon_id(&self) -> i32 {
        self.spell_icon_id as i32
    }

    pub const fn active_icon_id(&self) -> i32 {
        self.active_icon_id as i32
    }

    pub const fn spell_priority(&self) -> i32 {
        self.spell_priority as i32
    }

    pub const fn spell_name(&self) -> &'static str {
        self.spell_name
    }

    pub const fn rank_text(&self) -> &'static str {
        self.rank_text
    }

    pub const fn mana_cost_percentage(&self) -> i32 {
        self.mana_cost_percentage as i32
    }

    pub const fn start_recovery_category(&self) -> i32 {
        self.start_recovery_category as i32
    }

    pub const fn start_recovery_time(&self) -> i32 {
        self.start_recovery_time as i32
    }

    pub const fn max_target_level(&self) -> i32 {
        match self.entry {
            26259 => 20,
            26258 => 30,
            453 | 2908 | 26195 => 40,
            10267 | 26197 => 50,
            8192 | 8955 => 55,
            9439 | 9976 | 9991 | 9999 | 10723 | 12332 | 17085 | 21544 | 21565 => 60,
            9901 | 10953 | 26198 => 70,
            10268 => 80,
            9736 => 100,
            9773 | 9779 => 255,
            _ => 0,
        }
    }

    pub const fn spell_family_name(&self) -> i32 {
        self.spell_family_name as i32
    }

    pub const fn spell_family_flags(&self) -> i64 {
        self.spell_family_flags
    }

    pub const fn max_affected_targets(&self) -> i32 {
        self.max_affected_targets as i32
    }

    pub const fn dmg_class(&self) -> i32 {
        self.dmg_class as i32
    }

    pub const fn prevention_type(&self) -> i32 {
        self.prevention_type as i32
    }

    pub const fn stance_bar_order(&self) -> i32 {
        self.stance_bar_order as i32
    }

    pub const fn min_faction_id(&self) -> i32 {
        match self.entry {
            6994 => 369,
            _ => 0,
        }
    }

    pub const fn min_reputation(&self) -> i32 {
        match self.entry {
            6994 => 4,
            _ => 0,
        }
    }

    pub const fn required_aura_vision(&self) -> i32 {
        match self.entry {
            26869 => 1,
            _ => 0,
        }
    }

    pub const fn is_server_side(&self) -> i32 {
        0
    }

    pub const fn attributes_serverside(&self) -> i32 {
        match self.entry {
            4044 | 4133 | 11816 | 18115 | 21789 | 27791 | 28330 => 4,
            _ => 0,
        }
    }

    pub const fn reagents(&self) -> &[Reagent; 8] {
        &self.reagents
    }

    pub const fn effects(&self) -> &[SpellEffect; 3] {
        &self.effects
    }

    pub const fn totems(&self) -> &[Totem; 2] {
        &self.totems
    }

}
#[derive(Debug, Copy, Clone)]
pub struct Reagent {
    pub reagent: i32,
    pub amount: i32,
}

impl Reagent {
    pub const fn new(
        reagent: i32,
        amount: i32,
    ) -> Self {
        Self {
            reagent,
            amount,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct SpellEffect {
    pub effect: i32,
    pub die_sides: i32,
    pub base_dice: i32,
    pub dice_per_level: f32,
    pub real_points_per_level: f32,
    pub base_points: i32,
    pub mechanic: i32,
    pub implicit_target_a: i32,
    pub implicit_target_b: i32,
    pub radius_index: i32,
    pub apply_aura_name: i32,
    pub amplitude: i32,
    pub multiple_value: f32,
    pub chain_target: i32,
    pub item_type: u32,
    pub misc_value: i32,
    pub trigger_spell: i32,
    pub effect_points_per_combo_point: f32,
    pub damage_multiplier: f32,
}

impl SpellEffect {
    pub const fn new(
        effect: i32,
        die_sides: i32,
        base_dice: i32,
        dice_per_level: f32,
        real_points_per_level: f32,
        base_points: i32,
        mechanic: i32,
        implicit_target_a: i32,
        implicit_target_b: i32,
        radius_index: i32,
        apply_aura_name: i32,
        amplitude: i32,
        multiple_value: f32,
        chain_target: i32,
        item_type: u32,
        misc_value: i32,
        trigger_spell: i32,
        effect_points_per_combo_point: f32,
        damage_multiplier: f32,
    ) -> Self {
        Self {
            effect,
            die_sides,
            base_dice,
            dice_per_level,
            real_points_per_level,
            base_points,
            mechanic,
            implicit_target_a,
            implicit_target_b,
            radius_index,
            apply_aura_name,
            amplitude,
            multiple_value,
            chain_target,
            item_type,
            misc_value,
            trigger_spell,
            effect_points_per_combo_point,
            damage_multiplier,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Totem {
    pub totem: i32,
}

impl Totem {
    pub const fn new(
        totem: i32,
    ) -> Self {
        Self {
            totem,
        }
    }
}
// AUTOGENERATED_END