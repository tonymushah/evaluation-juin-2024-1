-- Your SQL goes here
create table temps_coureur(
    id_temps_coureur UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    temps INT,
    points INT,
    equipe_coureur UUID NOT NULL REFERENCES equipe_coureur(id_equipe_coureur),
    etape INT NOT NULL REFERENCES etape(rang)
);