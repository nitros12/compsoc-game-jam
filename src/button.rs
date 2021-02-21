use bevy::prelude::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(button_system.system())
            .add_event::<ButtonPressedEvent>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonState {
    Pressed,
    Released,
}

struct ButtonPressedEvent(Entity);

fn button_system(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &Handle<ColorMaterial>,
            &mut ButtonState,
            &Children,
        ),
        (Mutated<Interaction>, With<Button>),
    >,
    mut ev_click: ResMut<Events<ButtonPressedEvent>>,
) {
    for (entity, interaction, material, mut state, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                let c = materials.get_mut(material).unwrap();
                c.color.set_r(0.6);
                c.color.set_g(0.6);
                c.color.set_b(0.6);
            }
            Interaction::Hovered => {
                let c = materials.get_mut(material).unwrap();
                c.color.set_r(0.8);
                c.color.set_g(0.8);
                c.color.set_b(0.8);
            }
            Interaction::None => {
                let c = materials.get_mut(material).unwrap();
                c.color.set_r(0.8);
                c.color.set_g(0.8);
                c.color.set_b(0.8);
            }
        }

        match *interaction {
            Interaction::Clicked if *state == ButtonState::Released => {
                *state = ButtonState::Pressed;
            }
            _ if *state == ButtonState::Pressed => {
                *state = ButtonState::Released;

                ev_click.send(ButtonPressedEvent(entity))
            }
            _ => {}
        }
    }
}
