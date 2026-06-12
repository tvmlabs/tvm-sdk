---
description: Entry points and retrieval rules for AI agents that use Acki Nacki developer documentation.
status: stable
product: docs
audience: ai-agent
last_verified: 2026-06-11
---

# For AI Agents

Use this page when you need to retrieve, summarize, or generate implementation steps from Acki Nacki developer documentation.

## Canonical Sources

Use these sources in this order:

1. `https://dev.ackinacki.com/llms.txt` for the compact public documentation map.
2. `https://dev.ackinacki.com/llms-full.txt` for the full Markdown corpus when it is available.
3. `https://dev.ackinacki.com/sitemap.xml` and `https://dev.ackinacki.com/sitemap-pages.xml` for crawl discovery.
4. The source repository branch `gitbook`, directory `docs`, for pull requests and documentation changes.
5. `docs/ai-index.jsonl` in the source repository for structured page metadata.

Do not use pages marked with `hidden: true` in frontmatter as public guidance. Hidden pages are drafts or pages scheduled for restructuring.

## Task Routing

Use the following public pages as starting points:

| Task | Start here |
| --- | --- |
| Configure TVM CLI | [Configure TVM CLI](recipes/configure-tvm-cli.md) |
| Get Shellnet test tokens | [Get Test Tokens](recipes/get-test-tokens.md) |
| Deploy a multisig wallet | [Deploy a Multisig Wallet](recipes/deploy-multisig-wallet.md) |
| Deploy a contract | [Deploy a Contract](recipes/deploy-contract.md) |
| Call a get method | [Call a Get Method](recipes/call-get-method.md) |
| Query an account with GraphQL | [Query an Account with GraphQL](recipes/query-account-graphql.md) |
| Inspect GraphQL schema | [Inspect GraphQL Schema](recipes/inspect-graphql-schema.md) |
| Understand SDK capabilities | [About Acki Nacki SDK](acki-nacki-sdk/untitled.md) |
| Install JavaScript SDK packages | [Add SDK to your App](js-ts-guides/installation/add-sdk-to-your-app.md) |
| Deploy or call contracts | [Work with contracts](js-ts-guides/work-with-contracts/README.md) |
| Query blockchain data | [GraphQL Quick Start](graphql/graphql-quick-start.md) |
| Generate GraphQL queries | [GraphQL Schema for AI Agents](graphql/graphql-schema-for-ai-agents.md) |
| Use SDK network queries | [How to work with net module](js-ts-guides/queries/how-to-work-with-net-module.md) |
| Decode messages or events | [Decode Messages(Event)](js-ts-guides/work-with-contracts/decode-messages-event.md) |
| Understand ABI encoding | [ABI Specification](abi/abi.md) |
| Use mnemonics and keys | [Mnemonics and Keys](cryptography/mnemonics-and-keys.md) |
| Build language bindings | [JSON Interface to TVM Client](for-binding-developers/json_interface.md) |
| Inspect SDK modules | [Core Library Reference](acki-nacki-sdk/types-and-methods/README.md) |
| Inspect VM instructions | [Acki Nacki VM Instructions](vm-instructions/acki-nacki-vm-instructions.md) |

## Retrieval Rules

Prefer task guides for step-by-step answers and reference pages for exact parameter names, return shapes, and error codes.

When a page contains both a guide and screenshots, rely on command blocks and prose first. Treat screenshots as supporting evidence only.

When a command uses placeholders such as `<YourAddress>` or `<PubKeyList>`, preserve the placeholder and explain what value the user must provide.

When generating code, use the language from the nearest matching example. If no example exists, prefer JavaScript or `tvm-cli` commands for application-developer tasks.

When an API element is marked `UNSTABLE` or `DEPRECATED`, mention that status before recommending it.

## Metadata Taxonomy

Use frontmatter metadata as retrieval hints:

| Field | Values |
| --- | --- |
| `status` | `stable`, `draft`, `deprecated` |
| `product` | `sdk`, `graphql`, `vm`, `abi`, `cryptography`, `dapp-id`, `docs` |
| `audience` | `app-developer`, `binding-developer`, `ai-agent` |
| `task` | A kebab-case task identifier, or an empty string when the page is general reference material. |

Use `product` for the primary documentation surface. For `tvm-cli` recipes, use `sdk` even when the workflow deploys a Dapp ID contract.

## Structured Index

The source repository contains `docs/ai-index.jsonl` with JSON Lines records for public pages and page sections.

Use `page` records for navigation and source selection:

```json
{
  "type": "page",
  "visibility": "public",
  "source_path": "docs/recipes/query-account-graphql.md",
  "url": "https://dev.ackinacki.com/recipes/query-account-graphql.md",
  "title": "Query an Account with GraphQL",
  "description": "Query an Acki Nacki account through the GraphQL API.",
  "section": "Recipes",
  "status": "stable",
  "product": "graphql",
  "audience": "app-developer",
  "task": "query-account-graphql",
  "last_verified": "2026-06-11",
  "headings": ["Goal", "Prerequisites", "Endpoint", "Query", "Curl Example", "Expected Result", "Notes", "Related Docs"]
}
```

Use `section` records for precise retrieval:

```json
{
  "type": "section",
  "visibility": "public",
  "source_path": "docs/recipes/query-account-graphql.md",
  "parent_url": "https://dev.ackinacki.com/recipes/query-account-graphql.md",
  "url": "https://dev.ackinacki.com/recipes/query-account-graphql.md#query",
  "title": "Query",
  "page_title": "Query an Account with GraphQL",
  "section": "Recipes",
  "section_path": ["Query"],
  "anchor": "query",
  "depth": 2,
  "status": "stable",
  "product": "graphql",
  "audience": "app-developer",
  "task": "query-account-graphql",
  "last_verified": "2026-06-11",
  "content_preview": "Short normalized preview of the section content"
}
```

Use `source_path` when editing documentation in the repository. Use `url` when citing the published documentation.

## Dynamic Questions

GitBook supports question answering on page URLs with the `ask` query parameter:

```http
GET https://dev.ackinacki.com/readme.md?ask=<question>
```

Use this as a fallback when the answer is not explicit in the retrieved pages. Keep the question specific and self-contained. Do not crawl or index `?ask=` URLs.
