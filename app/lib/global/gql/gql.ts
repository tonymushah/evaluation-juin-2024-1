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
    "\n\tquery classementEquipeGenerale($ordre: GraphQLOrdering = DESCENDING, $pagination: OffsetLimit) {\n\t\tclassements {\n\t\t\tparEquipe(ordre: $ordre, pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tequipe\n\t\t\t\t\tnom\n\t\t\t\t\ttemps\n\t\t\t\t\tpoints\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n": types.ClassementEquipeGeneraleDocument,
    "\n\tquery classementGenerale($ordre: GraphQLOrdering, $pagination: OffsetLimit) {\n\t\tclassements {\n\t\t\tparCoureur(ordre: $ordre, pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tcoureur\n\t\t\t\t\ttemps\n\t\t\t\t\tpoints\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n": types.ClassementGeneraleDocument,
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
export function graphql(source: "\n\tquery classementEquipeGenerale($ordre: GraphQLOrdering = DESCENDING, $pagination: OffsetLimit) {\n\t\tclassements {\n\t\t\tparEquipe(ordre: $ordre, pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tequipe\n\t\t\t\t\tnom\n\t\t\t\t\ttemps\n\t\t\t\t\tpoints\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery classementEquipeGenerale($ordre: GraphQLOrdering = DESCENDING, $pagination: OffsetLimit) {\n\t\tclassements {\n\t\t\tparEquipe(ordre: $ordre, pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tequipe\n\t\t\t\t\tnom\n\t\t\t\t\ttemps\n\t\t\t\t\tpoints\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n\tquery classementGenerale($ordre: GraphQLOrdering, $pagination: OffsetLimit) {\n\t\tclassements {\n\t\t\tparCoureur(ordre: $ordre, pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tcoureur\n\t\t\t\t\ttemps\n\t\t\t\t\tpoints\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"): (typeof documents)["\n\tquery classementGenerale($ordre: GraphQLOrdering, $pagination: OffsetLimit) {\n\t\tclassements {\n\t\t\tparCoureur(ordre: $ordre, pagination: $pagination) {\n\t\t\t\tdata {\n\t\t\t\t\tcoureur\n\t\t\t\t\ttemps\n\t\t\t\t\tpoints\n\t\t\t\t}\n\t\t\t\tlimit\n\t\t\t\toffset\n\t\t\t\ttotal\n\t\t\t}\n\t\t}\n\t}\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;