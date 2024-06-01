-- Your SQL goes here
create table coureur(
    numero_dosard SERIAL PRIMARY KEY,
    nom TEXT NOT NULL,
    genre INT NOT NULL,
    dtn DATE NOT NULL
);