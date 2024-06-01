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

diesel::table! {
    coureur_categorie (id_coureur_categorie) {
        id_coureur_categorie -> Uuid,
        categorie -> Uuid,
        coureur -> Int4,
    }
}

diesel::table! {
    equipe (id_equipe) {
        id_equipe -> Uuid,
        pseudo -> Text,
        mot_passe -> Text,
        nom -> Text,
    }
}

diesel::joinable!(coureur_categorie -> categorie (categorie));
diesel::joinable!(coureur_categorie -> coureur (coureur));

diesel::allow_tables_to_appear_in_same_query!(
    categorie,
    coureur,
    coureur_categorie,
    equipe,
);
