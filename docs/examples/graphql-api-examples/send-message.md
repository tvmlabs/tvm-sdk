# Send message

Use mutation `postRequests` for this.

You can send a batch of messages with this API.

Use [encode\_message](../../reference/types-and-methods/mod\_abi.md#encode\_message) function to generate a message.

Use  [get\_boc\_hash](../../reference/types-and-methods/mod\_boc.md#get\_boc\_hash) function to calculate message hash.

```graphql
mutation{
  postRequests(requests:[
    {
      id: "tvm-hash-of-message-boc-in-base64"
      body: "message-body-in-base64"
    }
  ])
}
```
