-- Your SQL goes here
create table equipe(
    id_equipe UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pseudo TEXT NOT NULL,
    mot_passe TEXT NOT NULL,
    nom TEXT NOT NULL
);
