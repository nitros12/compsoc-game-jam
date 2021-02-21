use bevy::prelude::*;

pub fn average_colours<I: Iterator<Item = Color>>(it: I) -> Color {
    let mut total = 0;
    let (r, g, b) = it.fold((0.0, 0.0, 0.0), |(r, g, b), colour| {
        total += 1;

        (
            r + colour.r().powi(2),
            g + colour.g().powi(2),
            b + colour.b().powi(2),
        )
    });

    if total > 0 {
        Color::rgb(
            (r / total as f32).sqrt(),
            (g / total as f32).sqrt(),
            (b / total as f32).sqrt(),
        )
    } else {
        Color::rgba(0.0, 0.0, 0.0, 0.0)
    }
}
