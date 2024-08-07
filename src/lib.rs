use wasm_bindgen::prelude::*;
use csv::ReaderBuilder;
use serde_json::Value;
use reqwest;
use web_sys::console;use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub poster: String,
    pub title: String,
    pub year: String,
    pub released: String,
    pub rated: String,
    pub runtime: String,
    pub rating1: String,
    pub rating2: String,
    pub rating3: String,
    pub genre: String,
    pub director: String,
    pub writer: String,
    pub actors: String,
    pub plot: String,
    pub language: String,
    pub country: String,
    pub boxoffice: String
}

#[wasm_bindgen]
pub fn filter_csv(csv_data: &str, query: &str) -> String {
    let mut result = String::new();
    let query_lower = query.to_lowercase();
    
    let mut rdr = ReaderBuilder::new().from_reader(csv_data.as_bytes());
    for record in rdr.records() {
        let record = record.unwrap(); // Handle errors as needed
        if record.iter().any(|field| field.to_lowercase().contains(&query_lower)) {
            result.push_str(&record.iter().collect::<Vec<_>>().join(","));
            result.push('\n');
        }
    }
    
    result  
}

#[wasm_bindgen]
pub fn sort_csv(csv_data: &str, column_index: usize, sort_order: &str) -> String {
    let mut lines = csv_data.lines().collect::<Vec<_>>();

    // Check if there is any data
    if lines.is_empty() {
        return String::new(); // Return empty string if no data
    }

    // Extract the header row
    let header = lines.remove(0);

    // Convert remaining lines to records (data rows)
    let mut records: Vec<Vec<String>> = lines
        .iter()
        .map(|&line| line.split(',').map(String::from).collect())
        .collect();

    // Ensure column_index is within bounds
    if column_index >= records.get(0).unwrap_or(&vec![]).len() {
        return String::new(); // Return empty string if column_index is out of bounds
    }

    // Sort the data rows
    records.sort_by(|a, b| {
        let a_val = a.get(column_index).unwrap_or(&String::new()).clone();
        let b_val = b.get(column_index).unwrap_or(&String::new()).clone();
        
        let cmp = match (a_val.parse::<f64>(), b_val.parse::<f64>()) {
            (Ok(a_num), Ok(b_num)) => a_num.partial_cmp(&b_num).unwrap_or(std::cmp::Ordering::Equal),
            _ => a_val.cmp(&b_val),
        };
        
        if sort_order == "desc" {
            cmp.reverse()
        } else {
            cmp
        }
    });
    

    // Combine header with sorted data
    let sorted_csv = records
        .iter()
        .map(|row| row.join(","))
        .collect::<Vec<_>>()
        .join("\n");

    format!("{}\n{}", header, sorted_csv)
}

#[wasm_bindgen]
pub async fn fetch_movie(title: String, year: Option<i32>) -> Result<JsValue, JsValue> {
    let api_key = "f95d2eac";
    let url = format!(
        "https://www.omdbapi.com/?t={}&y={}&apikey={}",
        title,
        year.unwrap_or_default(), // Use default value if year is None
        api_key
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let text = response.text().await.map_err(|err| JsValue::from_str(&err.to_string()))?;
    let v: Value = serde_json::from_str(&text).map_err(|err| JsValue::from_str(&err.to_string()))?;

    let empty_vec = vec![];
    let ratings_array = v["Ratings"].as_array().unwrap_or(&empty_vec);

    let mut ratings = ratings_array.iter().map(|rating| rating["Value"].as_str().unwrap_or("").to_string()).collect::<Vec<_>>();

    // Ensure at least three ratings are present
    ratings.resize(3, String::new());

    let movie = Movie {
        poster: v["Poster"].as_str().unwrap_or_default().into(),
        title: v["Title"].as_str().unwrap_or_default().into(),
        year: v["Year"].as_str().unwrap_or_default().into(),
        rated: v["Rated"].as_str().unwrap_or_default().into(),
        runtime: v["Runtime"].as_str().unwrap_or_default().into(),
        rating1: ratings.get(0).cloned().unwrap_or_default(),
        rating2: ratings.get(1).cloned().unwrap_or_default(),
        rating3: ratings.get(2).cloned().unwrap_or_default(),
        released: v["Released"].as_str().unwrap_or_default().into(),
        genre: v["Genre"].as_str().unwrap_or_default().into(),
        director: v["Director"].as_str().unwrap_or_default().into(),
        writer: v["Writer"].as_str().unwrap_or_default().into(),
        actors: v["Actors"].as_str().unwrap_or_default().into(),
        plot: v["Plot"].as_str().unwrap_or_default().into(),
        language: v["Language"].as_str().unwrap_or_default().into(),
        country: v["Country"].as_str().unwrap_or_default().into(),
        boxoffice: v["Box Office"].as_str().unwrap_or_default().into()
    };

    // Convert movie to CSV format
    let csv_row = movie_to_csv(&movie);

    // Log CSV row to the console
    console::log_1(&format!("{}", csv_row).into());

    // Return the CSV row as a String
    Ok(JsValue::from_str(&csv_row))
}

fn replace_empty_fields(value: &str) -> String {
    if value.is_empty() {
        "N/A".to_string()
    } else {
        value.to_string()
    }
}

fn movie_to_csv(movie: &Movie) -> String {
    format!(
        "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
        replace_empty_fields(&movie.poster),
        replace_empty_fields(&movie.title),
        replace_empty_fields(&movie.year),
        replace_empty_fields(&movie.rated),
        replace_empty_fields(&movie.runtime),
        replace_empty_fields(&movie.rating1),
        replace_empty_fields(&movie.rating2),
        replace_empty_fields(&movie.rating3),
        replace_empty_fields(&movie.released),
        replace_empty_fields(&movie.genre),
        replace_empty_fields(&movie.director),
        replace_empty_fields(&movie.writer),
        replace_empty_fields(&movie.actors),
        replace_empty_fields(&movie.plot),
        replace_empty_fields(&movie.language),
        replace_empty_fields(&movie.country),
        replace_empty_fields(&movie.boxoffice)
    )
}
