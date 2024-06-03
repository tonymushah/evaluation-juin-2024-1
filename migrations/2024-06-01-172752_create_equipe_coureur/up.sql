-- Your SQL goes here
create table equipe_coureur(
    id_equipe_coureur UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coureur INT NOT NULL UNIQUE REFERENCES coureur(numero_dosard),
    equipe UUID NOT NULL REFERENCES equipe(id_equipe)
);