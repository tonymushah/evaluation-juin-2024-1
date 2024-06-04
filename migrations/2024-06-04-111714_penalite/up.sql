-- Your SQL goes here
CREATE TABLE penalite (
	id_penalite UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	etape INT NOT NULL REFERENCES etape(rang),
	equipe UUID NOT NULL REFERENCES equipe(id_equipe),
	valeur INT NOT NULL
);

CREATE VIEW V_PENALITE as 
SELECT 
	etape,
	equipe,
	SUM(valeur) as valeur
FROM penalite GROUP BY etape, equipe;