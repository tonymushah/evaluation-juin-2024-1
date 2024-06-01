diesel::table! {
    v_temps_coureur_etape(etape, equipe_coureur) {
        etape -> Int4,
        longueur -> Numeric,
        nom -> Text,
        nb_coureur_par_equipe -> Int4,
        temps -> Int4,
        points -> Int4,
        equipe_coureur -> Uuid
    }
}

diesel::table! {
    v_temps_coureur_etape_equipe_coureur(etape, equipe_coureur, coureur, equipe) {
        etape -> Int4,
        longueur -> Numeric,
        nom -> Text,
        nb_coureur_par_equipe -> Int4,
        temps -> Int4,
        points -> Int4,
        equipe_coureur -> Uuid,
        coureur -> Uuid,
        equipe -> Uuid
    }
}
