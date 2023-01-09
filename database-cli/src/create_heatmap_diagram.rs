//! Functions for the cli sub-command `create_heatmap`

use std::collections::HashMap;

use futures::StreamExt;
use plotters::prelude::*;

/// This function is called by the cli sub-command `create_heatmap`.
///
/// It creates a svg diagram from the data in the `location_counts` table in the database.
/// The diagram can be filtered by a regex for `cat_out` and a smaller-or-equal-than clause for `cat_code`.
/// See [`draw_diagram`] about the diagram specifics.
pub async fn create_heatmap(
    output_file: Option<&String>,
    filter_cat_out: &String,
    filter_cat_code: &u8,
    limit_search: &i64,
    limit_entries: &usize,
) {
    let mut conn = crate::util::database::connect().await;
    let mut stream = sqlx::query!(
        "SELECT name,count FROM lookup_data.location_counts 
        WHERE cat_code <= $1
        AND cat_out ~* $2
        ORDER BY count DESC 
        LIMIT $3",
        i16::from(*filter_cat_code),
        filter_cat_out,
        *limit_search
    )
    .fetch(&mut conn);

    //Create HashMap for counting
    let mut data: HashMap<String, u32> = HashMap::new();
    println!("Requesting data.");

    while let Some(record) = stream.next().await.map(|v| v.unwrap()) {
        if let Some(value) = data.get_mut(&record.name) {
            *value += record.count as u32;
        } else {
            data.insert(record.name, record.count as u32);
        }
    }

    //Map HashMap to Vec
    let mut data: Vec<(String, u32)> = data.into_iter().collect();

    //Sort by count
    data.sort_by(|(_, last), (_, next)| next.partial_cmp(last).unwrap());

    //Truncate Vec to maximum entrie
    data.truncate(*limit_entries);

    println!("Creating diagram from {} entries", data.len());
    //Use data to create diagram
    draw_diagram(output_file.map(|v| v.as_str()), &data);
}

/// Draws a horizontal bar diagram from the input Vec to a given svg file.
/// 
/// The file defaults to `./heatmap.svg`.
/// Caption of the chart is always `Train visits in database`.
/// If `limit_entries` is set higher than 11, the bar names get messed up.
pub fn draw_diagram(output_file: Option<&str>, data: &[(String, u32)]) {
    let max_value = data.iter().fold(0, |acc, (_, x)| acc.max(*x));
    let columns: Vec<String> = data.iter().map(|(v, _)| v.clone()).collect();

    let root_area = SVGBackend::new(
        output_file.unwrap_or("./heatmap.svg"),
        ((450 + 120) as u32, (data.len() * 20 + 30) as u32),
    )
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 120)
        .set_label_area_size(LabelAreaPosition::Bottom, 20)
        .caption("Train visits in database", ("sans-serif", 10))
        .build_cartesian_2d(0..(max_value + 100), columns.into_segmented())
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        Histogram::horizontal(&ctx)
            .margin(3)
            .data(data.iter().map(|(x, y)| (SegmentValue::Exact(x), *y)))
            .style(BLUE.filled()),
    )
    .unwrap();
}
