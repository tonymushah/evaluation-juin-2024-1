-- Your SQL goes here
create table categorie(
    id_categorie UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL
);