use json5::from_str;
use pgrx::*;
use serde_json::Value as JsonValue;
use tantivy::aggregation::agg_req::Aggregations;
use tantivy::aggregation::agg_result::AggregationResults;
use tantivy::aggregation::AggregationCollector;
use tantivy::query::AllQuery;

use crate::index_access::utils::get_parade_index;

#[pg_extern]
pub fn aggregation(index_name: &str, query: &str) -> JsonB {
    // Get Parade index
    let parade_index = get_parade_index(index_name.to_string());

    // Initialize aggregation searcher + collector
    let searcher = parade_index.searcher();
    let agg_req: Aggregations = from_str(query).expect("error parsing query");
    let collector = AggregationCollector::from_aggs(agg_req, Default::default());

    // Collect aggregation results
    let agg_res: AggregationResults = searcher
        .search(&AllQuery, &collector)
        .expect("error collecting aggregation results");
    let res: JsonValue = serde_json::to_value(agg_res).unwrap();

    JsonB(res)
}
