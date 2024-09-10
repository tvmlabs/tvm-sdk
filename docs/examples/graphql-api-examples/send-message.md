# Send message

Use mutation `postRequests` for this.

You can send a batch of messages with this API.

Use [this function](https://docs.everos.dev/ever-sdk/reference/types-and-methods/mod\_boc#get\_boc\_hash) to calculate message hash.

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
