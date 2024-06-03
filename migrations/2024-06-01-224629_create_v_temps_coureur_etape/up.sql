-- Your SQL goes here
CREATE VIEW V_TEMPS_COUREUR_ETAPE AS
SELECT
	E.RANG AS ETAPE,
	E.LONGUEUR,
	E.NOM,
	E.NB_COUREUR_PAR_EQUIPE,
	TC.TEMPS,
	TC.POINTS,
	TC.EQUIPE_COUREUR,
	E.DEPART
FROM
	TEMPS_COUREUR AS TC
	JOIN ETAPE AS E ON TC.ETAPE = E.RANG;