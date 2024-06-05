/* eslint-disable */
import * as types from './graphql';
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n\tquery listAdminEtape($pagination: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tdepart\n\t\t\t\t\tfinished\n\t\t\t\t\trang\n\t\t\t\t\tdepart\n\t\t\t\t\tnom\n\t\t\t\t\tlongueur\n\t\t\t\t\tcoureur(pagination: { limit: 1, offset: 0 }) {\n\t\t\t\t\t\ttotal\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\toffset\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n": types.ListAdminEtapeDocument,
    "\n\tquery selectEquipe($page: OffsetLimit!) {\n\t\tequipe {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tnom\n\t\t\t\t\tidEquipe\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t\toffset\n\t\t\t}\n\t\t}\n\t}\n": types.SelectEquipeDocument,
    "\n\tquery selectEtape($page: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tnom\n\t\t\t\t\trang\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t\toffset\n\t\t\t}\n\t\t}\n\t}\n": types.SelectEtapeDocument,
    "\n\tquery penalites($page: OffsetLimit!) {\n\t\tpenalite {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tidPenalite\n\t\t\t\t\tetapeData {\n\t\t\t\t\t\tnom\n\t\t\t\t\t}\n\t\t\t\t\tequipeData {\n\t\t\t\t\t\tnom\n\t\t\t\t\t}\n\t\t\t\t\tvaleur\n\t\t\t\t}\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t\tlimit\n\t\t\t}\n\t\t}\n\t}\n": types.PenalitesDocument,
    "\n\tmutation generateCat {\n\t\tgenerateCategories\n\t}\n": types.GenerateCatDocument,
    "\n\tquery getAdminEtape($rang: Int!, $courPage: OffsetLimit!) {\n\t\tetape {\n\t\t\tunique(rang: $rang) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\trang\n\t\t\t\tnom\n\t\t\t\tnbCoureurParEquipe\n\t\t\t\tlongueur\n\t\t\t\tcoureur(pagination: $courPage) {\n\t\t\t\t\tdata {\n\t\t\t\t\t\ttemps\n\t\t\t\t\t\tpoints\n\t\t\t\t\t\tcoureur {\n\t\t\t\t\t\t\tnumeroDosard\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t\tequipeCoureur\n\t\t\t\t\t\tequipe {\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tlimit\n\t\t\t\t\ttotal\n\t\t\t\t\toffset\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n": types.GetAdminEtapeDocument,
    "\n\tmutation addTimeToJoueur($etape: Int!, $dosard: Int!, $temps: TempsCoureur!) {\n\t\tetape(id: $etape) {\n\t\t\taddTime(dosard: $dosard, temps: $temps) {\n\t\t\t\ttemps\n\t\t\t\tetape\n\t\t\t\tidTempsCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n": types.AddTimeToJoueurDocument,
    "\n\tmutation importPoints($file: Upload!) {\n\t\timport {\n\t\t\tpoints(file: $file) {\n\t\t\t\trang\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n": types.ImportPointsDocument,
    "\n\tmutation importEtapes($file: Upload!) {\n\t\timport {\n\t\t\tetapes(file: $file) {\n\t\t\t\trang\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\tnbCoureurParEquipe\n\t\t\t}\n\t\t}\n\t}\n": types.ImportEtapesDocument,
    "\n\tmutation importResultats($file: Upload!) {\n\t\timport {\n\t\t\tresultats(file: $file) {\n\t\t\t\tidTempsCoureur\n\t\t\t\ttemps\n\t\t\t\tpoints\n\t\t\t\tequipeCoureur\n\t\t\t\tetape\n\t\t\t}\n\t\t}\n\t}\n": types.ImportResultatsDocument,
    "\n\tmutation addPenaliteAction($input: PenaliteInput!) {\n\t\tpenalite {\n\t\t\tupsert(input: $input) {\n\t\t\t\tetape\n\t\t\t\tequipe\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n": types.AddPenaliteActionDocument,
    "\n\tmutation removePenaliteAction($input: UUID!) {\n\t\tpenalite {\n\t\t\tremove(id: $input) {\n\t\t\t\tetape\n\t\t\t\tequipe\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n": types.RemovePenaliteActionDocument,
    "\n\tmutation resetDB {\n\t\tresetDb\n\t}\n": types.ResetDbDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery listAdminEtape($pagination: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tdepart\n\t\t\t\t\tfinished\n\t\t\t\t\trang\n\t\t\t\t\tdepart\n\t\t\t\t\tnom\n\t\t\t\t\tlongueur\n\t\t\t\t\tcoureur(pagination: { limit: 1, offset: 0 }) {\n\t\t\t\t\t\ttotal\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\toffset\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery listAdminEtape($pagination: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tdepart\n\t\t\t\t\tfinished\n\t\t\t\t\trang\n\t\t\t\t\tdepart\n\t\t\t\t\tnom\n\t\t\t\t\tlongueur\n\t\t\t\t\tcoureur(pagination: { limit: 1, offset: 0 }) {\n\t\t\t\t\t\ttotal\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t\toffset\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery selectEquipe($page: OffsetLimit!) {\n\t\tequipe {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tnom\n\t\t\t\t\tidEquipe\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t\toffset\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery selectEquipe($page: OffsetLimit!) {\n\t\tequipe {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tnom\n\t\t\t\t\tidEquipe\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t\toffset\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery selectEtape($page: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tnom\n\t\t\t\t\trang\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t\toffset\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery selectEtape($page: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tnom\n\t\t\t\t\trang\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\ttotal\n\t\t\t\toffset\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery penalites($page: OffsetLimit!) {\n\t\tpenalite {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tidPenalite\n\t\t\t\t\tetapeData {\n\t\t\t\t\t\tnom\n\t\t\t\t\t}\n\t\t\t\t\tequipeData {\n\t\t\t\t\t\tnom\n\t\t\t\t\t}\n\t\t\t\t\tvaleur\n\t\t\t\t}\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t\tlimit\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery penalites($page: OffsetLimit!) {\n\t\tpenalite {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\tidPenalite\n\t\t\t\t\tetapeData {\n\t\t\t\t\t\tnom\n\t\t\t\t\t}\n\t\t\t\t\tequipeData {\n\t\t\t\t\t\tnom\n\t\t\t\t\t}\n\t\t\t\t\tvaleur\n\t\t\t\t}\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t\tlimit\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation generateCat {\n\t\tgenerateCategories\n\t}\n"): (typeof documents)["\n\tmutation generateCat {\n\t\tgenerateCategories\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery getAdminEtape($rang: Int!, $courPage: OffsetLimit!) {\n\t\tetape {\n\t\t\tunique(rang: $rang) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\trang\n\t\t\t\tnom\n\t\t\t\tnbCoureurParEquipe\n\t\t\t\tlongueur\n\t\t\t\tcoureur(pagination: $courPage) {\n\t\t\t\t\tdata {\n\t\t\t\t\t\ttemps\n\t\t\t\t\t\tpoints\n\t\t\t\t\t\tcoureur {\n\t\t\t\t\t\t\tnumeroDosard\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t\tequipeCoureur\n\t\t\t\t\t\tequipe {\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tlimit\n\t\t\t\t\ttotal\n\t\t\t\t\toffset\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery getAdminEtape($rang: Int!, $courPage: OffsetLimit!) {\n\t\tetape {\n\t\t\tunique(rang: $rang) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\trang\n\t\t\t\tnom\n\t\t\t\tnbCoureurParEquipe\n\t\t\t\tlongueur\n\t\t\t\tcoureur(pagination: $courPage) {\n\t\t\t\t\tdata {\n\t\t\t\t\t\ttemps\n\t\t\t\t\t\tpoints\n\t\t\t\t\t\tcoureur {\n\t\t\t\t\t\t\tnumeroDosard\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t\tequipeCoureur\n\t\t\t\t\t\tequipe {\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tlimit\n\t\t\t\t\ttotal\n\t\t\t\t\toffset\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation addTimeToJoueur($etape: Int!, $dosard: Int!, $temps: TempsCoureur!) {\n\t\tetape(id: $etape) {\n\t\t\taddTime(dosard: $dosard, temps: $temps) {\n\t\t\t\ttemps\n\t\t\t\tetape\n\t\t\t\tidTempsCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tmutation addTimeToJoueur($etape: Int!, $dosard: Int!, $temps: TempsCoureur!) {\n\t\tetape(id: $etape) {\n\t\t\taddTime(dosard: $dosard, temps: $temps) {\n\t\t\t\ttemps\n\t\t\t\tetape\n\t\t\t\tidTempsCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation importPoints($file: Upload!) {\n\t\timport {\n\t\t\tpoints(file: $file) {\n\t\t\t\trang\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tmutation importPoints($file: Upload!) {\n\t\timport {\n\t\t\tpoints(file: $file) {\n\t\t\t\trang\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation importEtapes($file: Upload!) {\n\t\timport {\n\t\t\tetapes(file: $file) {\n\t\t\t\trang\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\tnbCoureurParEquipe\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tmutation importEtapes($file: Upload!) {\n\t\timport {\n\t\t\tetapes(file: $file) {\n\t\t\t\trang\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\tnbCoureurParEquipe\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation importResultats($file: Upload!) {\n\t\timport {\n\t\t\tresultats(file: $file) {\n\t\t\t\tidTempsCoureur\n\t\t\t\ttemps\n\t\t\t\tpoints\n\t\t\t\tequipeCoureur\n\t\t\t\tetape\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tmutation importResultats($file: Upload!) {\n\t\timport {\n\t\t\tresultats(file: $file) {\n\t\t\t\tidTempsCoureur\n\t\t\t\ttemps\n\t\t\t\tpoints\n\t\t\t\tequipeCoureur\n\t\t\t\tetape\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation addPenaliteAction($input: PenaliteInput!) {\n\t\tpenalite {\n\t\t\tupsert(input: $input) {\n\t\t\t\tetape\n\t\t\t\tequipe\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tmutation addPenaliteAction($input: PenaliteInput!) {\n\t\tpenalite {\n\t\t\tupsert(input: $input) {\n\t\t\t\tetape\n\t\t\t\tequipe\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation removePenaliteAction($input: UUID!) {\n\t\tpenalite {\n\t\t\tremove(id: $input) {\n\t\t\t\tetape\n\t\t\t\tequipe\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tmutation removePenaliteAction($input: UUID!) {\n\t\tpenalite {\n\t\t\tremove(id: $input) {\n\t\t\t\tetape\n\t\t\t\tequipe\n\t\t\t\tvaleur\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation resetDB {\n\t\tresetDb\n\t}\n"): (typeof documents)["\n\tmutation resetDB {\n\t\tresetDb\n\t}\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;