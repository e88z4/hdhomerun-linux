use hdhomerun_backend::models::{ApiErrorResponse, RememberedContext};
use proptest::prelude::*;

fn remembered_context_strategy() -> impl Strategy<Value = RememberedContext> {
    (
        prop::option::of("[a-z0-9\\-]{1,16}"),
        prop::option::of("[0-9]{1,3}(\\.[0-9]{1,3})?"),
        any::<bool>(),
        "[0-9TZ:\\-]{10,32}",
    )
        .prop_map(|(device_ref, channel_ref, auto_resume, updated_at)| RememberedContext {
            device_ref,
            channel_ref,
            auto_resume,
            updated_at,
        })
}

proptest! {
    #[test]
    fn remembered_context_round_trips_through_json(context in remembered_context_strategy()) {
        let encoded = serde_json::to_string(&context).expect("serialize context");
        let decoded: RememberedContext = serde_json::from_str(&encoded).expect("deserialize context");
        prop_assert_eq!(decoded, context);
    }

    #[test]
    fn api_error_invariants_hold(
        code in "[a-z_]{3,24}",
        message in ".{1,80}",
        retry_hint in ".{1,80}"
    ) {
        let error = ApiErrorResponse::new(code.clone(), message.clone(), retry_hint.clone());

        prop_assert_eq!(error.code, code);
        prop_assert_eq!(error.message, message);
        prop_assert_eq!(error.retry_hint, retry_hint);
        prop_assert!(error.details.is_none());
    }
}