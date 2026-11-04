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
    let mut last_error = None;
    for attempt in 0..3 {
        match client.query_with_vars(query, variables.clone()).await {
            Ok(data) => return Ok(data),
            Err(error) => {
                last_error = Some(error.to_string());
                if attempt < 2 {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
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