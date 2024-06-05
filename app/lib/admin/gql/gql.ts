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
    "\n\tquery getAdminEtape($rang: Int!, $courPage: OffsetLimit!) {\n\t\tetape {\n\t\t\tunique(rang: $rang) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\trang\n\t\t\t\tnom\n\t\t\t\tnbCoureurParEquipe\n\t\t\t\tlongueur\n\t\t\t\tcoureur(pagination: $courPage) {\n\t\t\t\t\tdata {\n\t\t\t\t\t\ttemps\n\t\t\t\t\t\tpoints\n\t\t\t\t\t\tcoureur {\n\t\t\t\t\t\t\tnumeroDosard\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t\tequipeCoureur\n\t\t\t\t\t\tequipe {\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tlimit\n\t\t\t\t\ttotal\n\t\t\t\t\toffset\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n": types.GetAdminEtapeDocument,
    "\n\tmutation addTimeToJoueur($etape: Int!, $dosard: Int!, $temps: TempsCoureur!) {\n\t\tetape(id: $etape) {\n\t\t\taddTime(dosard: $dosard, temps: $temps) {\n\t\t\t\ttemps\n\t\t\t\tetape\n\t\t\t\tidTempsCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n": types.AddTimeToJoueurDocument,
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
export function graphql(source: "\n\tquery getAdminEtape($rang: Int!, $courPage: OffsetLimit!) {\n\t\tetape {\n\t\t\tunique(rang: $rang) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\trang\n\t\t\t\tnom\n\t\t\t\tnbCoureurParEquipe\n\t\t\t\tlongueur\n\t\t\t\tcoureur(pagination: $courPage) {\n\t\t\t\t\tdata {\n\t\t\t\t\t\ttemps\n\t\t\t\t\t\tpoints\n\t\t\t\t\t\tcoureur {\n\t\t\t\t\t\t\tnumeroDosard\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t\tequipeCoureur\n\t\t\t\t\t\tequipe {\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tlimit\n\t\t\t\t\ttotal\n\t\t\t\t\toffset\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery getAdminEtape($rang: Int!, $courPage: OffsetLimit!) {\n\t\tetape {\n\t\t\tunique(rang: $rang) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\trang\n\t\t\t\tnom\n\t\t\t\tnbCoureurParEquipe\n\t\t\t\tlongueur\n\t\t\t\tcoureur(pagination: $courPage) {\n\t\t\t\t\tdata {\n\t\t\t\t\t\ttemps\n\t\t\t\t\t\tpoints\n\t\t\t\t\t\tcoureur {\n\t\t\t\t\t\t\tnumeroDosard\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t\tequipeCoureur\n\t\t\t\t\t\tequipe {\n\t\t\t\t\t\t\tnom\n\t\t\t\t\t\t}\n\t\t\t\t\t}\n\t\t\t\t\tlimit\n\t\t\t\t\ttotal\n\t\t\t\t\toffset\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation addTimeToJoueur($etape: Int!, $dosard: Int!, $temps: TempsCoureur!) {\n\t\tetape(id: $etape) {\n\t\t\taddTime(dosard: $dosard, temps: $temps) {\n\t\t\t\ttemps\n\t\t\t\tetape\n\t\t\t\tidTempsCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tmutation addTimeToJoueur($etape: Int!, $dosard: Int!, $temps: TempsCoureur!) {\n\t\tetape(id: $etape) {\n\t\t\taddTime(dosard: $dosard, temps: $temps) {\n\t\t\t\ttemps\n\t\t\t\tetape\n\t\t\t\tidTempsCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation resetDB {\n\t\tresetDb\n\t}\n"): (typeof documents)["\n\tmutation resetDB {\n\t\tresetDb\n\t}\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;