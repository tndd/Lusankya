// @generated automatically by Diesel CLI.

diesel::table! {
    asset (id, version) {
        id -> Uuid,
        version -> Text,
        class -> Text,
        exchange -> Text,
        symbol -> Text,
        status -> Text,
        tradable -> Bool,
        marginable -> Bool,
        shortable -> Bool,
        easy_to_borrow -> Bool,
        fractionable -> Bool,
    }
}
