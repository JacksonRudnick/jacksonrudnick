use reqwest_graphql::Client;
use serde::Deserialize;
use std::collections::HashMap;

pub(crate) async fn fetch_contributions(
    client: &Client<'_>,
    username: &str,
) -> Result<Data, Box<dyn std::error::Error>> {
    let query = r#"
    query($username: String!) {
        user(login: $username) {
            contributionsCollection {
                contributionCalendar {
                    weeks {
                        contributionDays {
                            contributionCount
                        }
                    }
                }
            }
        }
    }
    "#;
    let variables = HashMap::from([("username", username)]);
    const MAX_ATTEMPTS: u32 = 6;
    const BASE_DELAY_SECS: u64 = 2;

    let mut last_error = None;
    for attempt in 0..MAX_ATTEMPTS {
        match client.query_with_vars(query, variables.clone()).await {
            Ok(data) => return Ok(data),
            Err(error) => {
                last_error = Some(error.to_string());
                if attempt + 1 < MAX_ATTEMPTS {
                    let backoff = BASE_DELAY_SECS.saturating_mul(1u64 << attempt);
                    eprintln!(
                        "GitHub GraphQL request failed (attempt {}/{}): {}. Retrying in {}s...",
                        attempt + 1,
                        MAX_ATTEMPTS,
                        last_error.as_deref().unwrap_or("Unknown GraphQL error"),
                        backoff
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(backoff)).await;
                }
            }
        }
    }

    Err(std::io::Error::other(last_error.unwrap_or_else(|| "Unknown GraphQL error".to_string())).into())
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub user: User,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub contributions_collection: ContributionsCollection,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContributionsCollection {
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContributionCalendar {
    pub weeks: Vec<Week>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Week {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContributionDay {
    pub contribution_count: u32,
}