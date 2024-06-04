-- Your SQL goes here
CREATE TABLE penalite (
	id_penalite UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	etape INT NOT NULL REFERENCES etape(rang),
	equipe UUID NOT NULL REFERENCES equipe(id_equipe),
	valeur INT NOT NULL
);
