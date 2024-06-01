-- Your SQL goes here
create table coureur_categorie(
    id_coureur_categorie UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    categorie UUID NOT NULL REFERENCES categorie(id_categorie),
    coureur INT NOT NULL REFERENCES coureur(numero_dosard)
);