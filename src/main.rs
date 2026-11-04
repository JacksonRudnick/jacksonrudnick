mod contributions;
mod conway;
mod draw;

use contributions::fetch_contributions;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let endpoint = "https://api.github.com/graphql";
    let token = std::env::var("GITHUB_TOKEN")?;
    let authorization = format!("Bearer {token}");
    let headers = HashMap::from([
        ("Authorization", authorization.as_str()),
        ("User-Agent", "jacksonrudnick"),
    ]);
    let client = reqwest_graphql::Client::new_with_headers(endpoint, headers);
    let contributions = fetch_contributions(&client).await?;

    let weeks = contributions.viewer.contributions_collection.contribution_calendar.weeks;

    let columns = weeks
        .iter()
        .map(|week| week.contribution_days.len())
        .max()
        .unwrap_or(0);
    let grid = weeks
        .iter()
        .map(|week| {
            let mut row = week
                .contribution_days
                .iter()
                .map(|day| day.contribution_count > 0)
                .collect::<Vec<bool>>();
            row.resize(columns, false);
            row
        })
        .collect::<Vec<Vec<bool>>>();

    let history = conway::game_of_life(&grid);

    draw::save_history(&history, "history-light.svg", false);
    draw::save_history(&history, "history-dark.svg", true);

    Ok(())
}
