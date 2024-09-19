# Blockchain API

`blockchain` root type is API that includes such basic real-time data as:

* blocks
* key blocks
* transactions
* account
  * account info
  * account transactions
  * account messages
* (coming soon) accounts - allows to paginate accounts by deploy time or init time, filtering by code\_hash

This API is truly a Graph-oriented API .

We followed GraphQL best practices and implemented Relay Cursor Connections Specification for pagination for all lists. You can read more here [https://relay.dev/graphql/connections.htm](https://relay.dev/graphql/connections.htm)

{% hint style="info" %}
**Note**: With  by default Blockchain API provides blocks, transactions and messages data only for the past 7 days. For use cases where earlier data is needed make sure to use the `archive: true` flag in `blockchain` query filters. Do not however use it, if archive data isn't required, as it will generate unnecessary load.

Accounts data is still available in full. You do not need the `archive` flag to get data of accounts created more than 7 days ago.
{% endhint %}
