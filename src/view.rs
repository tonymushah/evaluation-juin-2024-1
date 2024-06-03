diesel::table! {
    v_temps_coureur_etape(etape, equipe_coureur) {
        etape -> Int4,
        longueur -> Numeric,
        nom -> Text,
        nb_coureur_par_equipe -> Int4,
        temps -> Nullable<Int4>,
        points -> Nullable<Int4>,
        equipe_coureur -> Uuid,
        depart -> Timestamp
    }
}

diesel::table! {
    v_temps_coureur_etape_equipe_coureur(etape, equipe_coureur, coureur, equipe) {
        etape -> Int4,
        longueur -> Numeric,
        nom -> Text,
        nb_coureur_par_equipe -> Int4,
        temps -> Nullable<Int4>,
        points -> Nullable<Int4>,
        equipe_coureur -> Uuid,
        coureur -> Int4,
        equipe -> Uuid,
        depart -> Timestamp
    }
}

diesel::table! {
    v_equipe_coureur(coureur, equipe, id_equipe_coureur) {
        coureur -> Int4,
        equipe -> Uuid,
        nom_coureur -> Text,
        genre -> Int4,
        dtn -> Date,
        pseudo_equipe -> Text,
        nom_equipe -> Text,
        id_equipe_coureur -> Uuid
    }
}

diesel::table! {
    v_coureur_point(coureur) {
        coureur -> Int4,
        points -> Numeric,
        temps -> Numeric
    }
}

diesel::table! {
    v_equipe_point(equipe) {
        equipe -> Int4,
        points -> Numeric,
        temps -> Numeric
    }
}

diesel::table! {
    v_coureur_point_etape(coureur, etape) {
        coureur -> Int4,
        points -> Numeric,
        temps -> Numeric,
        etape -> Int4
    }
}

diesel::table! {
    v_equipe_point_etape(equipe, etape) {
        equipe -> Int4,
        points -> Numeric,
        temps -> Numeric,
        etape -> Int4
    }
}

diesel::table! {
    v_coureur_categorie(id_cc, categorie, coureur) {
        id_cc -> Uuid,
        categorie -> Uuid,
        coureur -> Integer,
        nom -> Text,
        genre -> Integer,
        dtn -> Date,
        designation -> Text
    }
}

diesel::table! {
    v_temps_coureur_etape_equipe_coureur_categorie(etape, equipe_coureur, coureur, equipe, categorie, id_cc) {
        etape -> Integer,
        longueur -> Numeric,
        etape_nom -> Text,
        nb_par_equipe -> Integer,
        temps -> Nullable<Numeric>,
        points -> Nullable<Numeric>,
        equipe -> Uuid,
        equipe_coureur -> Uuid,
        coureur -> Uuid,
        nom_coureur -> Text,
        genre -> Integer,
        categorie -> Uuid,
        id_cc -> Uuid,
        dtn -> Date,
        designation -> Text
    }
}

diesel::table! {
    v_classement_categorie(coureur, categorie) {
        coureur -> Int4,
        categorie -> Uuid,
        points -> Numeric,
        temps -> Numeric
    }
}

diesel::table! {
    v_classement_categories_equipe(coureur, categorie, equipe) {
        coureur -> Int4,
        categorie -> Uuid,
        equipe -> Uuid,
        points -> Numeric,
        temps -> Numeric
    }
}
