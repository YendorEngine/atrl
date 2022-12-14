use crate::{prelude::*, systems::AppState, ui::*};

pub fn ui_widget_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut q_widget: Query<&mut UIWidget>,
    state: Res<CurrentState<AppState>>,
) -> bool {
    let state_entity = widget_context.use_state(&mut commands, entity, UIWidget::default());
    if let Ok(mut ui_widget) = q_widget.get_mut(state_entity) {
        if keys.just_pressed(KeyCode::Escape) {
            ui_widget.show_ui = !ui_widget.show_ui;
        }

        let parent_id = Some(entity);
        rsx! {
            <ElementBundle>
                {if ui_widget.show_ui{
                    constructor! {
                        <WindowBundle
                            window={KWindow {
                                draggable: true,
                                size: Vec2::new(512.0, 512.0),
                                title: "Layout example".into(),
                                initial_position: Vec2::new(10.0, 10.0),
                                ..KWindow::default()
                            }}
                        >
                            <ElementBundle>
                                {
                                    match state.0 {
                                        AppState::Initializing => {
                                            constructor!{
                                                <TextWidgetBundle
                                                    text={TextProps {
                                                        content: "In Initializing state".into(),
                                                        ..default()
                                                    }}
                                                />
                                            }
                                        }
                                        AppState::Loading(..) => {
                                            constructor!{
                                                <TextWidgetBundle
                                                    text={TextProps {
                                                        content: "In Loading state(s)".into(),
                                                        ..default()
                                                    }}
                                                />
                                            }
                                        }
                                        AppState::Menu(..) => {
                                            constructor!{
                                                <TextWidgetBundle
                                                    text={TextProps {
                                                        content: "In Menu state(s)".into(),
                                                        ..default()
                                                    }}
                                                />
                                            }
                                        }
                                        AppState::InGame => {
                                            constructor!{
                                                <TextWidgetBundle
                                                    text={TextProps {
                                                        content: "In InGame state".into(),
                                                        ..default()
                                                    }}
                                                />
                                            }
                                        }
                                        _ => {
                                            constructor!{
                                                <TextWidgetBundle
                                                    text={TextProps {
                                                        content: "In Quitting state".into(),
                                                        ..default()
                                                    }}
                                                />
                                            }
                                        }
                                    }
                                }
                            </ElementBundle>

                        </WindowBundle>
                    }
                }}
            </ElementBundle>
        };
    }

    true
}
