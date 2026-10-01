use scraper::{Html, Selector};

use crate::{
    api::dtos::opengraph_dtos::OpenGraphResponse,
    application::service::errors::OpenGraphServiceError,
};

pub struct OpenGraphService;

impl OpenGraphService {
    pub async fn fetch(
        url: &str,
    ) -> Result<OpenGraphResponse, OpenGraphServiceError> {
        let parsed_url = url::Url::parse(url).map_err(|_| OpenGraphServiceError::InvalidUrl)?;

        Self::validate_url(&parsed_url)?;

        let response = reqwest::Client::new()
            .get(parsed_url)
            .header(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (compatible; OpenGraphBot/1.0)",
            )
            .send()
            .await?;

        let final_url = response.url().to_string();

        let html = response
            .text()
            .await
            .map_err(|_| OpenGraphServiceError::ResponseReadFailed)?;

        let document = Html::parse_document(&html);

        let meta_selector =
            Selector::parse("meta").map_err(|_| OpenGraphServiceError::ParseFailed)?;

        let title_selector =
            Selector::parse("title").map_err(|_| OpenGraphServiceError::ParseFailed)?;

        let mut title = None;
        let mut description = None;
        let mut image = None;

        for element in document.select(&meta_selector) {
            let property = element
                .value()
                .attr("property")
                .or_else(|| element.value().attr("name"));

            let content = element.value().attr("content");

            let (Some(property), Some(content)) = (property, content) else {
                continue;
            };

            match property.to_lowercase().as_str() {
                "og:title" => {
                    title = Some(content.to_string());
                }

                "og:description" => {
                    description = Some(content.to_string());
                }

                "og:image" => {
                    image = Some(content.to_string());
                }

                _ => {}
            }
        }

        let title = title.or_else(|| {
            document
                .select(&title_selector)
                .next()
                .map(|element| element.text().collect::<String>())
        });

        let description = description.or_else(|| {
            document.select(&meta_selector).find_map(|element| {
                let name = element.value().attr("name")?;

                if name.eq_ignore_ascii_case("description") {
                    element.value().attr("content").map(String::from)
                } else {
                    None
                }
            })
        });

        let image = image.and_then(|image| {
            url::Url::parse(&final_url)
                .ok()
                .and_then(|base| base.join(&image).ok())
                .map(|url| url.to_string())
        });

        Ok(OpenGraphResponse {
            url: final_url,
            title,
            description,
            image,
        })
    }

    pub async fn fetch_image(url: &str) -> Result<reqwest::Response, OpenGraphServiceError> {
        let parsed_url = url::Url::parse(url).map_err(|_| OpenGraphServiceError::InvalidUrl)?;

        Self::validate_url(&parsed_url)?;

        let response = reqwest::Client::new()
            .get(parsed_url)
            .header(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (compatible; OpenGraphBot/1.0)",
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(OpenGraphServiceError::FetchFailed);
        }

        Ok(response)
    }

    fn validate_url(url: &url::Url) -> Result<(), OpenGraphServiceError> {
        match url.scheme() {
            "http" | "https" => Ok(()),
            _ => Err(OpenGraphServiceError::UnsupportedScheme),
        }
    }
}
