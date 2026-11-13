
use bevy::{color::palettes::tailwind::CYAN_300, prelude::*};

use crate::other_sppd::components::*;

pub fn resolve_material_state(
    mut q: Query<(
        &mut MeshMaterial3d<StandardMaterial>,
        &OriginalMaterial,
        Option<&BeingHovered>,
    )>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    let hover_matl = mats.add(Color::from(CYAN_300));

    for (mut mat, original, hovered) in q.iter_mut() {
        
        if hovered.is_some() {
            mat.0 = hover_matl.clone();
        } else {
            mat.0 = original.0.clone();
        }
    }
}


