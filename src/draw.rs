use svg::node::element::{Animate, Group, Rectangle};
use svg::Document;

fn render_history(history: &Vec<Vec<Vec<bool>>>, dark_mode: bool) -> Document {
    let rows = history.first().map_or(0, Vec::len);
    let cols = history
        .first()
        .and_then(|grid| grid.first())
        .map_or(0, Vec::len);
    let mut document = Document::new()
        .set("viewBox", (0, 0, rows, cols))
        .set("width", rows * 10)
        .set("height", cols * 10)
        .set("preserveAspectRatio", "xMidYMid meet");
    if history.is_empty() {
        return document;
    }
    let (background, inactive, active, active_stroke, inactive_stroke) = if dark_mode {
        ("#0b1220", "#1e293b", "#34d399", "#6ee7b7", "#263449")
    } else {
        ("#f8fafc", "#e2e8f0", "#16a34a", "#15803d", "#cbd5e1")
    };

    document = document.add(
        Rectangle::new()
            .set("width", cols)
            .set("height", rows)
            .set("fill", background)
            .set("transform", format!("rotate(90) translate(0, -{rows})")),
    );
    let frame_count = history.len();
    let duration = frame_count as f64 * 0.5;
    let key_times = (0..=frame_count)
        .map(|index| (index as f64 / frame_count as f64).to_string())
        .collect::<Vec<_>>()
        .join(";");

    for (step_index, grid) in history.iter().enumerate() {
        let mut frame =
            Group::new()
                .set("display", if step_index == 0 { "inline" } else { "none" })
                .set("transform", format!("rotate(90) translate(0, -{rows})"));
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                let rect = Rectangle::new()
                    .set("x", j as f64 + 0.08)
                    .set("y", i as f64 + 0.08)
                    .set("width", 0.84)
                    .set("height", 0.84)
                    .set("rx", 0.16)
                    .set("fill", if cell { active } else { inactive })
                    .set("stroke", if cell { active_stroke } else { inactive_stroke })
                    .set("stroke-width", 0.025);
                frame = frame.add(rect);
            }
        }
        let values = (0..=frame_count)
            .map(|index| if index == step_index { "inline" } else { "none" })
            .collect::<Vec<_>>()
            .join(";");
        frame = frame.add(
            Animate::new()
                .set("attributeName", "display")
                .set("values", values)
                .set("keyTimes", key_times.as_str())
                .set("calcMode", "discrete")
                .set("dur", format!("{duration}s"))
                .set("repeatCount", "indefinite"),
        );
        document = document.add(frame);
    }
    document
}

pub fn save_history(history: &Vec<Vec<Vec<bool>>>, filename: &str, dark_mode: bool) {
    let document = render_history(history, dark_mode);
    svg::save(filename, &document).unwrap();
}