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

diesel::table! {
    equipe_coureur (id_equipe_coureur) {
        id_equipe_coureur -> Uuid,
        coureur -> Int4,
        equipe -> Uuid,
    }
}

diesel::table! {
    etape (rang) {
        rang -> Int4,
        longueur -> Numeric,
        nom -> Text,
        nb_coureur_par_equipe -> Int4,
    }
}

diesel::joinable!(coureur_categorie -> categorie (categorie));
diesel::joinable!(coureur_categorie -> coureur (coureur));
diesel::joinable!(equipe_coureur -> coureur (coureur));
diesel::joinable!(equipe_coureur -> equipe (equipe));

diesel::allow_tables_to_appear_in_same_query!(
    categorie,
    coureur,
    coureur_categorie,
    equipe,
    equipe_coureur,
    etape,
);
