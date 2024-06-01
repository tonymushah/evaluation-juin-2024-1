-- Your SQL goes here
create table etape(
    rang SERIAL PRIMARY KEY,
    longueur DECIMAL NOT NULL,
    nom TEXT NOT NULL,
    nb_coureur_par_equipe INT NOT NULL
);