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
        points -> Int4,
        temps -> Int4
    }
}

diesel::table! {
    v_equipe_point(equipe) {
        equipe -> Int4,
        points -> Int4,
        temps -> Int4
    }
}

diesel::table! {
    v_coureur_point_etape(coureur, etape) {
        coureur -> Int4,
        points -> Int4,
        temps -> Int4,
        etape -> Int4
    }
}

diesel::table! {
    v_equipe_point_etape(equipe, etape) {
        equipe -> Int4,
        points -> Int4,
        temps -> Int4,
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
