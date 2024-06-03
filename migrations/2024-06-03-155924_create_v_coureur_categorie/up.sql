-- Your SQL goes here
create view v_coureur_categorie as
SELECT
	CC.ID_COUREUR_CATEGORIE AS ID_CC,
	CC.CATEGORIE,
	CC.COUREUR,
	crr.nom,
	crr.genre,
	crr.dtn,
	cat.designation
FROM
	COUREUR_CATEGORIE AS CC
	JOin coureur as crr on cc.coureur = crr.numero_dosard
	JOIN categorie as cat on cc.categorie = cat.id_categorie;
