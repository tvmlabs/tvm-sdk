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

Using ExtMsg API v2

```graphql
mutation {
  sendMessage(
		message: {
      id: "tvm-hash-of-message-boc-in-base64",
     	body: "message-body-in-base64",
      expireAt: "",
      threadId: ""
    }
  ) {
    aborted
    block_hash
    current_time
    message_hash
    producers
    thread_id
    tvm_exit_code
    tx_hash
  }
}
```