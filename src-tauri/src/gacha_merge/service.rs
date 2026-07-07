use std::collections::BTreeSet;

use tracing::info;

use crate::gacha_analysis::model::{GachaRecord, PoolFile};

use super::model::{PoolMergeResult, PoolMergeSummary};

pub fn merge_pool_files(
    old_pool_file: &PoolFile,
    new_pool_file: &PoolFile,
) -> (PoolFile, PoolMergeResult) {
    let mut merged_pool_file = PoolFile::new();
    let mut summaries = Vec::new();

    for pool_name in collect_pool_names(old_pool_file, new_pool_file) {
        let old_records = old_pool_file
            .get(&pool_name)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let new_records = new_pool_file
            .get(&pool_name)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let merged_records = merge_records(old_records, new_records);
        let appended_old_count = merged_records.len().saturating_sub(new_records.len());

        summaries.push(PoolMergeSummary {
            pool_name: pool_name.clone(),
            old_count: old_records.len(),
            new_count: new_records.len(),
            merged_count: merged_records.len(),
            appended_old_count,
        });
        merged_pool_file.insert(pool_name, merged_records);
    }

    let result = PoolMergeResult::from_summaries(summaries);

    info!(
        pool_count = result.summaries.len(),
        total_old_count = result.total_old_count,
        total_new_count = result.total_new_count,
        total_merged_count = result.total_merged_count,
        "抽卡记录合并完成"
    );

    (merged_pool_file, result)
}

fn collect_pool_names(old_pool_file: &PoolFile, new_pool_file: &PoolFile) -> BTreeSet<String> {
    old_pool_file
        .keys()
        .chain(new_pool_file.keys())
        .cloned()
        .collect()
}

fn merge_records(old_records: &[GachaRecord], new_records: &[GachaRecord]) -> Vec<GachaRecord> {
    if new_records.is_empty() {
        return old_records.to_vec();
    }

    if old_records.is_empty() {
        return new_records.to_vec();
    }

    // 参考实现按时间拼接，而不是按唯一 ID 去重。
    // new_records / old_records 均要求“最新在前”。取新数据最后一条的时间作为拼接边界。
    let new_last_time = &new_records[new_records.len() - 1].time;
    let mut result = new_records.to_vec();

    for (index, old_record) in old_records.iter().enumerate() {
        if old_record.time.as_str() < new_last_time.as_str() {
            result.extend_from_slice(&old_records[index..]);
            break;
        }

        if old_record.time == *new_last_time {
            for (same_time_index, old_record_same_time) in
                old_records.iter().enumerate().skip(index)
            {
                if old_record_same_time.time.as_str() < new_last_time.as_str() {
                    result.extend_from_slice(&old_records[same_time_index..]);
                    break;
                }
            }
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{merge_pool_files, merge_records};
    use crate::gacha_analysis::model::{GachaRecord, PoolFile};

    fn record(name: &str, time: &str) -> GachaRecord {
        GachaRecord {
            card_pool_type: "1".to_string(),
            resource_id: 1,
            quality_level: 3,
            resource_type: Some("武器".to_string()),
            name: name.to_string(),
            count: Some(1),
            time: time.to_string(),
        }
    }

    #[test]
    fn merge_records_should_return_old_when_new_is_empty() {
        let old_records = vec![record("old-1", "2024-01-01 10:00:00")];
        let result = merge_records(&old_records, &[]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "old-1");
    }

    #[test]
    fn merge_records_should_return_new_when_old_is_empty() {
        let new_records = vec![record("new-1", "2024-01-02 10:00:00")];
        let result = merge_records(&[], &new_records);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "new-1");
    }

    #[test]
    fn merge_records_should_append_older_old_records() {
        let new_records = vec![
            record("new-3", "2024-01-03 10:00:00"),
            record("new-2", "2024-01-02 10:00:00"),
        ];
        let old_records = vec![
            record("old-2-overlap", "2024-01-02 10:00:00"),
            record("old-1", "2024-01-01 10:00:00"),
        ];
        let result = merge_records(&old_records, &new_records);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].name, "new-3");
        assert_eq!(result[1].name, "new-2");
        assert_eq!(result[2].name, "old-1");
    }

    #[test]
    fn merge_records_should_skip_all_old_records_with_same_boundary_time() {
        let new_records = vec![
            record("new-3", "2024-01-03 10:00:00"),
            record("new-2-a", "2024-01-02 10:00:00"),
            record("new-2-b", "2024-01-02 10:00:00"),
        ];
        let old_records = vec![
            record("old-2-a", "2024-01-02 10:00:00"),
            record("old-2-b", "2024-01-02 10:00:00"),
            record("old-1", "2024-01-01 10:00:00"),
        ];
        let result = merge_records(&old_records, &new_records);

        assert_eq!(result.len(), 4);
        assert_eq!(result[0].name, "new-3");
        assert_eq!(result[1].name, "new-2-a");
        assert_eq!(result[2].name, "new-2-b");
        assert_eq!(result[3].name, "old-1");
    }

    #[test]
    fn merge_pool_files_should_merge_all_pool_names() {
        let mut old_pool_file = PoolFile::new();
        old_pool_file.insert(
            "角色活动唤取".to_string(),
            vec![record("old-1", "2024-01-01 10:00:00")],
        );
        old_pool_file.insert(
            "武器活动唤取".to_string(),
            vec![record("old-weapon", "2024-01-01 10:00:00")],
        );

        let mut new_pool_file = PoolFile::new();
        new_pool_file.insert(
            "角色活动唤取".to_string(),
            vec![record("new-2", "2024-01-02 10:00:00")],
        );

        let (merged, merge_result) = merge_pool_files(&old_pool_file, &new_pool_file);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged["角色活动唤取"].len(), 2);
        assert_eq!(merged["武器活动唤取"].len(), 1);
        assert_eq!(merge_result.total_old_count, 2);
        assert_eq!(merge_result.total_new_count, 1);
        assert_eq!(merge_result.total_merged_count, 3);
    }

    #[test]
    fn merge_pool_files_should_match_example_expected_file() {
        let old_pool_file: PoolFile =
            serde_json::from_str(include_str!("../../../doc/examples/merge-old-pool.json"))
                .expect("merge-old-pool.json should be valid");
        let new_pool_file: PoolFile =
            serde_json::from_str(include_str!("../../../doc/examples/merge-new-pool.json"))
                .expect("merge-new-pool.json should be valid");
        let expected_pool_file: PoolFile = serde_json::from_str(include_str!(
            "../../../doc/examples/merge-expected-pool.json"
        ))
        .expect("merge-expected-pool.json should be valid");

        let (merged, merge_result) = merge_pool_files(&old_pool_file, &new_pool_file);

        assert_eq!(merged.len(), expected_pool_file.len());
        assert_eq!(
            merged["角色活动唤取"].len(),
            expected_pool_file["角色活动唤取"].len()
        );
        assert_eq!(merged["角色活动唤取"][0].name, "忌炎");
        assert_eq!(merged["角色活动唤取"][1].name, "鉴心");
        assert_eq!(merged["角色活动唤取"][2].name, "戍关长刃·定军");
        assert_eq!(
            merged["常驻角色唤取"].len(),
            expected_pool_file["常驻角色唤取"].len()
        );
        assert_eq!(
            merged["武器活动唤取"].len(),
            expected_pool_file["武器活动唤取"].len()
        );
        assert_eq!(merge_result.total_old_count, 3);
        assert_eq!(merge_result.total_new_count, 3);
        assert_eq!(merge_result.total_merged_count, 5);
        assert_eq!(merge_result.total_appended_old_count, 2);
    }
}
