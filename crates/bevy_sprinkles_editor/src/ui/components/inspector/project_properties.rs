use bevy::prelude::*;

use crate::state::EditorState;
use crate::ui::widgets::checkbox::{CheckboxProps, checkbox};
use crate::ui::widgets::inspector_field::fields_row;
use crate::ui::widgets::text_edit::{TextEditProps, text_edit};

use super::{DynamicSectionContent, InspectorSection, inspector_section, section_needs_setup};
use crate::ui::components::binding::FieldBinding;
use crate::ui::components::inspector::FieldKind;

#[derive(Component)]
struct ProjectPropertiesSection;

#[derive(Component)]
struct ProjectPropertiesContent;

#[derive(Component)]
struct ProjectRuntimeSection;

#[derive(Component)]
struct ProjectRuntimeContent;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (setup_properties_content, setup_runtime_content));
}

pub fn project_properties_section(asset_server: &AssetServer) -> impl Bundle {
    (
        ProjectPropertiesSection,
        inspector_section(InspectorSection::new("Properties", vec![]), asset_server),
    )
}

pub fn project_runtime_section(asset_server: &AssetServer) -> impl Bundle {
    (
        ProjectRuntimeSection,
        inspector_section(InspectorSection::new("Runtime", vec![]), asset_server),
    )
}

fn setup_properties_content(
    mut commands: Commands,
    editor_state: Res<EditorState>,
    sections: Query<(Entity, &InspectorSection), With<ProjectPropertiesSection>>,
    existing: Query<Entity, With<ProjectPropertiesContent>>,
) {
    let Some(entity) = section_needs_setup(&sections, &existing) else {
        return;
    };

    if editor_state.current_project.is_none() {
        return;
    }

    let content = commands
        .spawn((
            ProjectPropertiesContent,
            DynamicSectionContent,
            Node {
                width: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(12.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(fields_row()).with_children(|row| {
                row.spawn((
                    FieldBinding::asset("name", FieldKind::String),
                    text_edit(TextEditProps::default().with_label("Project name")),
                ));
            });

            parent.spawn(fields_row()).with_children(|row| {
                row.spawn((
                    FieldBinding::asset("authors.submitted_by", FieldKind::String),
                    text_edit(TextEditProps::default().with_label("Submitted by")),
                ));
                row.spawn((
                    FieldBinding::asset("authors.inspired_by", FieldKind::String),
                    text_edit(TextEditProps::default().with_label("Inspired by")),
                ));
            });
        })
        .id();

    commands.entity(entity).add_child(content);
}

fn setup_runtime_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    editor_state: Res<EditorState>,
    sections: Query<(Entity, &InspectorSection), With<ProjectRuntimeSection>>,
    existing: Query<Entity, With<ProjectRuntimeContent>>,
) {
    let Some(entity) = section_needs_setup(&sections, &existing) else {
        return;
    };

    if editor_state.current_project.is_none() {
        return;
    }

    let content = commands
        .spawn((
            ProjectRuntimeContent,
            DynamicSectionContent,
            Node {
                width: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(12.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(fields_row()).with_children(|row| {
                row.spawn((
                    FieldBinding::asset("despawn_on_finish", FieldKind::Bool),
                    checkbox(CheckboxProps::new("Despawn on finish"), &asset_server),
                ));
            });
        })
        .id();

    commands.entity(entity).add_child(content);
}
