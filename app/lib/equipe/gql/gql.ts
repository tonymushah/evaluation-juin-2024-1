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
    "\n\tquery currentEtape($etape: Int!) {\n\t\tetape {\n\t\t\tunique(rang: $etape) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\tnom\n\t\t\t\trang\n\t\t\t\tlongueur\n\t\t\t}\n\t\t}\n\t}\n": types.CurrentEtapeDocument,
    "\n\tquery etapeCoureurs($etape: Int!) {\n\t\tetape {\n\t\t\tjoueurs(etape: $etape) {\n\t\t\t\tcoureur\n\t\t\t\tnomCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n": types.EtapeCoureursDocument,
    "\n\tquery listEtapes($page: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\trang\n\t\t\t\t\tlongueur\n\t\t\t\t\tnom\n\t\t\t\t\tdepart\n\t\t\t\t\tfinished\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n": types.ListEtapesDocument,
    "\n\tquery currentEquipe {\n\t\tcurrent {\n\t\t\tpseudo\n\t\t\tnom\n\t\t\tidEquipe\n\t\t}\n\t}\n": types.CurrentEquipeDocument,
    "\n\tmutation equipeLogin($user: String!, $password: String!) {\n\t\tlogin(username: $user, password: $password)\n\t}\n": types.EquipeLoginDocument,
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
export function graphql(source: "\n\tquery currentEtape($etape: Int!) {\n\t\tetape {\n\t\t\tunique(rang: $etape) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\tnom\n\t\t\t\trang\n\t\t\t\tlongueur\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery currentEtape($etape: Int!) {\n\t\tetape {\n\t\t\tunique(rang: $etape) {\n\t\t\t\tdepart\n\t\t\t\tfinished\n\t\t\t\tnom\n\t\t\t\trang\n\t\t\t\tlongueur\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery etapeCoureurs($etape: Int!) {\n\t\tetape {\n\t\t\tjoueurs(etape: $etape) {\n\t\t\t\tcoureur\n\t\t\t\tnomCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery etapeCoureurs($etape: Int!) {\n\t\tetape {\n\t\t\tjoueurs(etape: $etape) {\n\t\t\t\tcoureur\n\t\t\t\tnomCoureur\n\t\t\t\tpoints\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery listEtapes($page: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\trang\n\t\t\t\t\tlongueur\n\t\t\t\t\tnom\n\t\t\t\t\tdepart\n\t\t\t\t\tfinished\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery listEtapes($page: OffsetLimit!) {\n\t\tetape {\n\t\t\tlist(pagination: $page) {\n\t\t\t\tdata {\n\t\t\t\t\trang\n\t\t\t\t\tlongueur\n\t\t\t\t\tnom\n\t\t\t\t\tdepart\n\t\t\t\t\tfinished\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery currentEquipe {\n\t\tcurrent {\n\t\t\tpseudo\n\t\t\tnom\n\t\t\tidEquipe\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery currentEquipe {\n\t\tcurrent {\n\t\t\tpseudo\n\t\t\tnom\n\t\t\tidEquipe\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tmutation equipeLogin($user: String!, $password: String!) {\n\t\tlogin(username: $user, password: $password)\n\t}\n"): (typeof documents)["\n\tmutation equipeLogin($user: String!, $password: String!) {\n\t\tlogin(username: $user, password: $password)\n\t}\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;