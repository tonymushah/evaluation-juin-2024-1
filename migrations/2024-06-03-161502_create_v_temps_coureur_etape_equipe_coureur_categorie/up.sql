-- Your SQL goes here
Create view V_temps_coureur_etape_equipe_coureur_categorie as
SELECT
	TCEEC.ETAPE,
	TCEEC.LONGUEUR,
	TCEEC.NOM AS ETAPE_NOM,
	TCEEC.NB_COUREUR_PAR_EQUIPE AS NB_PAR_EQUIPE,
	TCEEC.TEMPS,
	TCEEC.POINTS,
	TCEEC.EQUIPE_COUREUR,
	TCEEC.COUREUR,
	TCEEC.EQUIPE,
	crr_cat.nom as nom_coureur,
	crr_cat.genre as genre,
	crr_cat.categorie as categorie,
	crr_cat.id_cc,
	crr_cat.dtn,
	crr_cat.designation
FROM
	V_coureur_categorie as crr_cat
	JOIN V_TEMPS_COUREUR_ETAPE_EQUIPE_COUREUR AS TCEEC
	On TCEEC.coureur = crr_cat.coureur;
