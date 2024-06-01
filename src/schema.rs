// @generated automatically by Diesel CLI.

diesel::table! {
    categorie (id_categorie) {
        id_categorie -> Uuid,
        designation -> Text,
    }
}

diesel::table! {
    coureur (numero_dosard) {
        numero_dosard -> Int4,
        nom -> Text,
        genre -> Int4,
        dtn -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categorie,
    coureur,
);
