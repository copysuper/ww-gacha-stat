use std::collections::HashSet;

use tracing::info;

use super::model::{
    AnalysisData, GachaRecord, HitData, PoolAnalysisSummary, PoolFile, PoolRankSummary,
};

pub fn analyze_pool_file(
    pool_file: &PoolFile,
    skip_first_ssr: bool,
    base_ssr_ids: &HashSet<String>,
) -> Vec<AnalysisData> {
    let mut results = Vec::with_capacity(pool_file.len());

    for (pool_name, records) in pool_file {
        results.push(analyze_one_pool(
            pool_name,
            records,
            skip_first_ssr,
            base_ssr_ids,
        ));
    }

    info!(pool_count = results.len(), "本地抽卡数据分析完成");

    results
}

pub fn build_pool_analysis_summaries(analysis_list: &[AnalysisData]) -> Vec<PoolAnalysisSummary> {
    analysis_list
        .iter()
        .map(build_pool_analysis_summary)
        .collect()
}

fn build_pool_analysis_summary(item: &AnalysisData) -> PoolAnalysisSummary {
    let ssr_event_count = item.ssr_data_list.iter().filter(|hit| hit.event).count();
    let ssr_permanent_count = item.ssr_data_list.len().saturating_sub(ssr_event_count);

    PoolAnalysisSummary {
        pool_name: item.pool_name.clone(),
        is_empty: item.is_empty,
        total_count: item.total_count,
        start_date: item.start_date.clone(),
        end_date: item.end_date.clone(),
        ssr: build_rank_summary(
            item.ssr_count,
            item.total_count,
            item.ssr_avg,
            item.ssr_min,
            item.ssr_max,
            item.no_up_ssr_count,
        ),
        sr: build_rank_summary(
            item.sr_count,
            item.total_count,
            item.sr_avg,
            item.sr_min,
            item.sr_max,
            item.no_up_sr_count,
        ),
        r: build_rank_summary(
            item.r_count,
            item.total_count,
            item.r_avg,
            item.r_min,
            item.r_max,
            item.no_up_r_count,
        ),
        ssr_event_count,
        ssr_permanent_count,
        latest_ssr: item.ssr_data_list.first().cloned(),
        latest_sr: item.sr_data_list.first().cloned(),
    }
}

fn analyze_one_pool(
    pool_name: &str,
    input_records: &[GachaRecord],
    skip_first_ssr: bool,
    base_ssr_ids: &HashSet<String>,
) -> AnalysisData {
    let mut records = input_records.to_vec();

    if skip_first_ssr {
        records = drop_oldest_ssr_and_older_records(&records);
    }

    let mut result = AnalysisData::empty(pool_name);
    result.total_count = records.len();

    if records.is_empty() {
        return result;
    }

    result.is_empty = false;
    result.start_date = records.last().map(|record| date_part(&record.time));
    result.end_date = records.first().map(|record| date_part(&record.time));

    let ssr_indexes = find_indexes(&records, 5);
    let sr_indexes = find_indexes(&records, 4);
    let r_indexes = find_indexes(&records, 3);

    result.ssr_count = ssr_indexes.len();
    result.sr_count = sr_indexes.len();
    result.r_count = r_indexes.len();

    result.no_up_ssr_count = ssr_indexes.first().copied().unwrap_or(records.len());
    result.no_up_sr_count = calc_no_up_sr_count(records.len(), &ssr_indexes, &sr_indexes);
    result.no_up_r_count = 0;

    result.ssr_data_list = build_hit_data_for_rank(&records, &ssr_indexes, 5, base_ssr_ids);
    result.sr_data_list = build_hit_data_for_rank(&records, &sr_indexes, 4, base_ssr_ids);
    result.r_data_list = Vec::new();

    fill_rank_stats(
        &mut result.ssr_avg,
        &mut result.ssr_min,
        &mut result.ssr_max,
        &result.ssr_data_list,
    );
    fill_rank_stats(
        &mut result.sr_avg,
        &mut result.sr_min,
        &mut result.sr_max,
        &result.sr_data_list,
    );

    result.r_avg = 0.0;
    result.r_min = 0;
    result.r_max = 0;

    result
}

fn drop_oldest_ssr_and_older_records(records: &[GachaRecord]) -> Vec<GachaRecord> {
    for i in (0..records.len()).rev() {
        if records[i].quality_level == 5 {
            return records[..i].to_vec();
        }
    }

    records.to_vec()
}

fn find_indexes(records: &[GachaRecord], quality_level: u8) -> Vec<usize> {
    records
        .iter()
        .enumerate()
        .filter_map(|(index, record)| (record.quality_level == quality_level).then_some(index))
        .collect()
}

fn calc_no_up_sr_count(total: usize, ssr_indexes: &[usize], sr_indexes: &[usize]) -> usize {
    let nearest_ssr_index = ssr_indexes.first().copied().unwrap_or(total);

    match sr_indexes.first().copied() {
        Some(nearest_sr_index) => nearest_sr_index.min(nearest_ssr_index),
        None => nearest_ssr_index,
    }
}

fn build_hit_data_for_rank(
    records: &[GachaRecord],
    indexes: &[usize],
    rank: u8,
    base_ssr_ids: &HashSet<String>,
) -> Vec<HitData> {
    let mut list = Vec::with_capacity(indexes.len());

    for (position, index) in indexes.iter().enumerate() {
        let next_index = indexes.get(position + 1).copied();
        let count = next_index.map_or(records.len() - *index, |value| value - *index);
        let record = &records[*index];

        list.push(HitData {
            id: record.resource_id,
            name: record.name.clone(),
            count,
            event: rank == 5 && !base_ssr_ids.contains(&record.resource_id.to_string()),
            date: record.time.clone(),
        });
    }

    list
}

fn fill_rank_stats(avg: &mut f64, min: &mut usize, max: &mut usize, list: &[HitData]) {
    if list.is_empty() {
        *avg = 0.0;
        *min = 0;
        *max = 0;
        return;
    }

    let counts: Vec<usize> = list.iter().map(|item| item.count).collect();
    let sum: usize = counts.iter().sum();

    *avg = sum as f64 / counts.len() as f64;
    *min = *counts.iter().min().unwrap_or(&0);
    *max = *counts.iter().max().unwrap_or(&0);
}

fn build_rank_summary(
    count: usize,
    total_count: usize,
    avg: f64,
    min: usize,
    max: usize,
    current_pity: usize,
) -> PoolRankSummary {
    if total_count == 0 {
        return PoolRankSummary::empty();
    }

    PoolRankSummary {
        count,
        rate: count as f64 / total_count as f64,
        avg,
        min,
        max,
        current_pity,
    }
}

fn date_part(time: &str) -> String {
    time.split(' ').next().unwrap_or(time).to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{analyze_pool_file, build_pool_analysis_summaries};
    use crate::gacha_analysis::model::PoolFile;

    fn default_base_ssr_ids() -> HashSet<String> {
        [
            "1104",
            "1203",
            "1301",
            "1503",
            "1405",
            "21010015",
            "21020015",
            "21030015",
            "21040015",
            "21050015",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    fn parse_pool_file(json: &str) -> PoolFile {
        serde_json::from_str(json).expect("样例 pool.json 应可正常解析")
    }

    fn assert_rate_close(actual: f64, expected: f64) {
        let diff = (actual - expected).abs();
        assert!(
            diff < 1e-9,
            "rate mismatch: actual={actual}, expected={expected}, diff={diff}"
        );
    }

    #[test]
    fn sample_pool_should_match_expected_summary() {
        let pool_file = parse_pool_file(include_str!("../../../doc/examples/sample-pool.json"));
        let analysis_list = analyze_pool_file(&pool_file, false, &default_base_ssr_ids());
        let summary_list = build_pool_analysis_summaries(&analysis_list);

        assert_eq!(summary_list.len(), 4);

        let role_event = summary_list
            .iter()
            .find(|item| item.pool_name == "角色活动唤取")
            .expect("应包含角色活动唤取");
        assert!(!role_event.is_empty);
        assert_eq!(role_event.total_count, 8);
        assert_eq!(role_event.start_date.as_deref(), Some("2026-07-06"));
        assert_eq!(role_event.end_date.as_deref(), Some("2026-07-06"));
        assert_eq!(role_event.ssr.count, 3);
        assert_eq!(role_event.ssr.current_pity, 2);
        assert_eq!(role_event.ssr.avg, 2.0);
        assert_eq!(role_event.ssr.min, 1);
        assert_eq!(role_event.ssr.max, 3);
        assert_rate_close(role_event.ssr.rate, 3.0 / 8.0);
        assert_eq!(role_event.sr.count, 2);
        assert_eq!(role_event.sr.current_pity, 0);
        assert_eq!(role_event.sr.avg, 4.0);
        assert_eq!(role_event.sr.min, 3);
        assert_eq!(role_event.sr.max, 5);
        assert_rate_close(role_event.sr.rate, 2.0 / 8.0);
        assert_eq!(role_event.r.count, 3);
        assert_rate_close(role_event.r.rate, 3.0 / 8.0);
        assert_eq!(role_event.ssr_event_count, 0);
        assert_eq!(role_event.ssr_permanent_count, 3);
        assert_eq!(role_event.latest_ssr.as_ref().map(|hit| hit.name.as_str()), Some("浩境粼光"));
        assert_eq!(role_event.latest_ssr.as_ref().map(|hit| hit.count), Some(3));
        assert_eq!(role_event.latest_sr.as_ref().map(|hit| hit.name.as_str()), Some("散华"));

        let weapon_event = summary_list
            .iter()
            .find(|item| item.pool_name == "武器活动唤取")
            .expect("应包含武器活动唤取");
        assert!(!weapon_event.is_empty);
        assert_eq!(weapon_event.total_count, 3);
        assert_eq!(weapon_event.ssr.count, 1);
        assert_eq!(weapon_event.ssr.current_pity, 0);
        assert_eq!(weapon_event.ssr.avg, 3.0);
        assert_eq!(weapon_event.sr.count, 1);
        assert_eq!(weapon_event.sr.current_pity, 0);
        assert_eq!(weapon_event.sr.avg, 2.0);
        assert_eq!(weapon_event.r.count, 1);
        assert_eq!(weapon_event.ssr_event_count, 0);
        assert_eq!(weapon_event.ssr_permanent_count, 1);
        assert_eq!(weapon_event.latest_ssr.as_ref().map(|hit| hit.name.as_str()), Some("停驻之烟"));

        for pool_name in ["角色常驻唤取", "武器常驻唤取"] {
            let empty_pool = summary_list
                .iter()
                .find(|item| item.pool_name == pool_name)
                .expect("应包含空卡池");
            assert!(empty_pool.is_empty);
            assert_eq!(empty_pool.total_count, 0);
            assert!(empty_pool.start_date.is_none());
            assert!(empty_pool.end_date.is_none());
        }
    }

    #[test]
    fn empty_pool_should_keep_all_zero_summary() {
        let pool_file = parse_pool_file(include_str!("../../../doc/examples/empty-pool.json"));
        let analysis_list = analyze_pool_file(&pool_file, false, &default_base_ssr_ids());
        let summary_list = build_pool_analysis_summaries(&analysis_list);

        assert_eq!(summary_list.len(), 4);

        for item in summary_list {
            assert!(item.is_empty);
            assert_eq!(item.total_count, 0);
            assert_eq!(item.ssr.count, 0);
            assert_eq!(item.sr.count, 0);
            assert_eq!(item.r.count, 0);
            assert!(item.latest_ssr.is_none());
            assert!(item.latest_sr.is_none());
        }
    }

    #[test]
    fn only_r_pool_should_keep_ssr_and_sr_empty() {
        let pool_file = parse_pool_file(include_str!("../../../doc/examples/only-r-pool.json"));
        let analysis_list = analyze_pool_file(&pool_file, false, &default_base_ssr_ids());
        let summary = build_pool_analysis_summaries(&analysis_list)
            .into_iter()
            .next()
            .expect("应至少有一个卡池");

        assert!(!summary.is_empty);
        assert_eq!(summary.total_count, 3);
        assert_eq!(summary.start_date.as_deref(), Some("2024-07-01"));
        assert_eq!(summary.end_date.as_deref(), Some("2024-07-01"));
        assert_eq!(summary.ssr.count, 0);
        assert_eq!(summary.ssr.current_pity, 3);
        assert_eq!(summary.sr.count, 0);
        assert_eq!(summary.sr.current_pity, 3);
        assert_eq!(summary.r.count, 3);
        assert_rate_close(summary.r.rate, 1.0);
        assert!(summary.latest_ssr.is_none());
        assert!(summary.latest_sr.is_none());
    }

    #[test]
    fn single_ssr_pool_should_mark_event_and_latest_ssr() {
        let pool_file = parse_pool_file(include_str!("../../../doc/examples/single-ssr-pool.json"));
        let analysis_list = analyze_pool_file(&pool_file, false, &default_base_ssr_ids());
        let summary = build_pool_analysis_summaries(&analysis_list)
            .into_iter()
            .next()
            .expect("应至少有一个卡池");

        assert!(!summary.is_empty);
        assert_eq!(summary.total_count, 3);
        assert_eq!(summary.ssr.count, 1);
        assert_eq!(summary.ssr.current_pity, 0);
        assert_eq!(summary.ssr.avg, 3.0);
        assert_eq!(summary.ssr.min, 3);
        assert_eq!(summary.ssr.max, 3);
        assert_eq!(summary.ssr_event_count, 1);
        assert_eq!(summary.ssr_permanent_count, 0);
        assert_eq!(summary.latest_ssr.as_ref().map(|hit| hit.name.as_str()), Some("今汐"));
        assert_eq!(summary.latest_ssr.as_ref().map(|hit| hit.count), Some(3));
    }

    #[test]
    fn skip_first_ssr_pool_should_drop_oldest_ssr_segment() {
        let pool_file = parse_pool_file(include_str!("../../../doc/examples/skip-first-ssr-pool.json"));
        let analysis_list = analyze_pool_file(&pool_file, true, &default_base_ssr_ids());
        let summary = build_pool_analysis_summaries(&analysis_list)
            .into_iter()
            .next()
            .expect("应至少有一个卡池");

        assert!(!summary.is_empty);
        assert_eq!(summary.total_count, 3);
        assert_eq!(summary.start_date.as_deref(), Some("2024-07-05"));
        assert_eq!(summary.end_date.as_deref(), Some("2024-07-05"));
        assert_eq!(summary.ssr.count, 1);
        assert_eq!(summary.ssr.current_pity, 0);
        assert_eq!(summary.ssr.avg, 3.0);
        assert_eq!(summary.ssr_event_count, 1);
        assert_eq!(summary.latest_ssr.as_ref().map(|hit| hit.name.as_str()), Some("忌炎"));
        assert_eq!(summary.sr.count, 1);
        assert_eq!(summary.sr.current_pity, 0);
        assert_eq!(summary.sr.avg, 2.0);
        assert_eq!(summary.r.count, 1);
    }
}
