use criterion::{Criterion, black_box, criterion_group, criterion_main};
use episodic::{
    ContinuationPolicyV2, OmContinuationCandidateV2, OmContinuationSourceKind,
    OmContinuationStateV2, parse_memory_section_xml_accuracy_first,
    parse_multi_thread_observer_output_accuracy_first, resolve_continuation_update,
};

fn bench_parse_memory_accuracy_first(c: &mut Criterion) {
    let malformed_overlap = concat!(
        "<observations>\n",
        "broken start\n",
        "<observations>\n",
        "good section\n",
        "</observations>\n",
        "<current-task>fix replay drift</current-task>\n",
        "<suggested-response>continue implementation</suggested-response>\n",
    );

    c.bench_function(
        "parse_memory_section_xml_accuracy_first/malformed_overlap",
        |b| b.iter(|| parse_memory_section_xml_accuracy_first(black_box(malformed_overlap))),
    );
}

fn bench_parse_multi_thread_accuracy_first(c: &mut Criterion) {
    let malformed_multi_thread = concat!(
        "<observations>\n",
        "<thread id=\"broken\">\n",
        "* malformed leading block\n",
        "<thread id=\"thread-2\">\n",
        "* valid thread observation\n",
        "<current-task>thread task</current-task>\n",
        "<suggested-response>thread response</suggested-response>\n",
        "</thread>\n",
        "</observations>\n",
    );

    c.bench_function(
        "parse_multi_thread_observer_output_accuracy_first/malformed_overlap",
        |b| {
            b.iter(|| {
                parse_multi_thread_observer_output_accuracy_first(black_box(malformed_multi_thread))
            })
        },
    );
}

fn bench_resolve_continuation_update(c: &mut Criterion) {
    let previous = OmContinuationStateV2 {
        scope_key: "thread:t-main".to_string(),
        thread_id: "t-main".to_string(),
        current_task: Some("stabilize replay indexing".to_string()),
        suggested_response: Some("respond with mitigation status".to_string()),
        confidence_milli: 920,
        source_kind: OmContinuationSourceKind::ObserverDeterministic,
        source_message_ids: vec!["m-prev-1".to_string(), "m-prev-2".to_string()],
        updated_at_rfc3339: "2026-03-05T00:00:00Z".to_string(),
        staleness_budget_ms: 15_000,
    };
    let candidate = OmContinuationCandidateV2 {
        scope_key: "thread:t-main".to_string(),
        thread_id: "t-main".to_string(),
        current_task: Some("stabilize replay indexing".to_string()),
        suggested_response: Some("respond with replay status update".to_string()),
        confidence_milli: 840,
        source_kind: OmContinuationSourceKind::ObserverDeterministic,
        source_message_ids: vec!["m-new-1".to_string(), "m-new-2".to_string()],
        updated_at_rfc3339: "2026-03-05T00:00:03Z".to_string(),
        staleness_budget_ms: 20_000,
    };
    let policy = ContinuationPolicyV2::default();

    c.bench_function("resolve_continuation_update/hotpath", |b| {
        b.iter(|| {
            resolve_continuation_update(
                black_box(Some(&previous)),
                black_box(&candidate),
                black_box(policy),
            )
        })
    });
}

criterion_group!(
    benches,
    bench_parse_memory_accuracy_first,
    bench_parse_multi_thread_accuracy_first,
    bench_resolve_continuation_update
);
criterion_main!(benches);
