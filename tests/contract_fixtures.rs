use episodic::{
    OM_PROMPT_CONTRACT_VERSION, OmPromptContractParseError, OmPromptRequestKind,
    parse_observer_prompt_contract_v2, parse_reflector_prompt_contract_v2,
};

fn fixture(path: &str) -> String {
    std::fs::read_to_string(path).expect("read fixture")
}

#[test]
fn observer_contract_fixture_valid_is_accepted() {
    let payload = fixture("tests/fixtures/contracts/observer_single.valid.json");
    let parsed =
        parse_observer_prompt_contract_v2(&payload, Some(OmPromptRequestKind::ObserverSingle))
            .expect("must parse");
    assert_eq!(
        parsed.header.request_kind,
        OmPromptRequestKind::ObserverSingle
    );
    assert_eq!(
        parsed.known_message_ids,
        vec!["m2".to_string(), "m1".to_string()]
    );
    assert!(parsed.output_contract.continuation_enabled);
}

#[test]
fn observer_contract_fixture_missing_scope_key_reports_diagnostic() {
    let payload = fixture("tests/fixtures/contracts/observer_single.missing_scope_key.json");
    let error =
        parse_observer_prompt_contract_v2(&payload, Some(OmPromptRequestKind::ObserverSingle))
            .expect_err("must fail");
    assert_eq!(
        error,
        OmPromptContractParseError::MissingRequiredField {
            field: "header.scope_key".to_string(),
        }
    );
}

#[test]
fn observer_contract_fixture_version_mismatch_reports_diagnostic() {
    let payload =
        fixture("tests/fixtures/contracts/observer_single.contract_version_mismatch.json");
    let error =
        parse_observer_prompt_contract_v2(&payload, Some(OmPromptRequestKind::ObserverSingle))
            .expect_err("must fail");
    assert_eq!(
        error,
        OmPromptContractParseError::ContractVersionMismatch {
            expected: OM_PROMPT_CONTRACT_VERSION.to_string(),
            actual: "9.9.9".to_string(),
        }
    );
}

#[test]
fn reflector_contract_fixture_valid_is_accepted() {
    let payload = fixture("tests/fixtures/contracts/reflector.valid.json");
    let parsed = parse_reflector_prompt_contract_v2(&payload).expect("must parse");
    assert_eq!(parsed.header.request_kind, OmPromptRequestKind::Reflector);
    assert_eq!(parsed.generation_count, 7);
    assert_eq!(
        parsed.output_contract.required_sections,
        vec!["observations"]
    );
}
