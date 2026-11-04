use reqwest_graphql::Client;
use serde::Deserialize;

pub(crate) async fn fetch_contributions(
    client: &Client<'_>,
) -> Result<Data, Box<dyn std::error::Error>> {
    let query = r#"
    query {
        viewer {
            contributionsCollection {
                contributionCalendar {
                    totalContributions
                    weeks {
                        contributionDays {
                            contributionCount
                            date
                        }
                    }
                }
            }
        }
    }
    "#;
    client
        .query(query)
        .await
        .map_err(|error| std::io::Error::other(error.to_string()).into())
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub viewer: Viewer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
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
    pub total_contributions: u32,
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
    pub date: String,
}