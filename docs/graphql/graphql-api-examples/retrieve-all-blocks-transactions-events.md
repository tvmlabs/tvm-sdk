# Retrieve all blocks, transactions, events

## Blocks pagination

### Get finalized timestamp

As the API is eventually consistant before starting pagination we need to limit the pagination range by the finalized timestamp (timestamp that guarantees that there is not missed objects before it).

```
query{
    blockchain{
        finalized_timestamp
    }
}
```



### What is сhain\_order?

Because Acki Nacki blockchain dynamically splits and merges it is not possible to follow one chain seqno to sequentially retrieve all the blocks.

We added a unique index for blocks to paginate them across all the threads: `chain_order`

```
chain_order = block-timestamp-in-seconds + 
                placeholder-for-future-purposes +
                thread_id + height
```

Each value should be converted to hex string and prefixed with \<string size -1>.

For example:&#x20;

chain\_order=`7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0`, where

`7698320d0` (8-length of  timestamp, timestamp value = 1770201296 or Wednesday, 4 February 2026 10:34:56

`00` - placeholder value for future usage with value 0 and length=1-1=0

`6700000000000000000000000000000000000000000000000000000000000000000000` - thread \`00000000000000000000000000000000000000000000000000000000000000000000 , length 68-1=67

`61d4b1c0` - height=30716352, length 7-1=6

### Paginate by blocks within timestamp range <a href="#paginate_by_seqno" id="paginate_by_seqno"></a>

Pagination parameters:&#x20;

```
master_seq_no_range:{
    start: 1770201296 # start timestamp
    end: 1770204896 # end timestamp <=finalized_timestamp !!!
 }
```

`after,before`     - specify chain\_order/cursor field here

Here we continue pagination within timestamp range and ask for the next 3 blocks after the last cursor in the previous query. We see that the next page exists so we can continue paginating within the same timestamp range.

```graphql
query{
  blockchain{
    blocks(
      master_seq_no_range:{
        start: 1770201296
        end: 1770204896
      }
      after:"7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0"
      first:3
    ){
      edges{
        node{
          id
          tr_count
          chain_order
        }
        cursor        
      }
      pageInfo{
        startCursor
        endCursor
        hasNextPage
      }
    }
  }
}
```

Result:

```
{
  "data": {
    "blockchain": {
      "blocks": {
        "edges": [
          {
            "node": {
              "id": "a040869b8cbeaeb7ab4b189e492d4fbd4b58e2694a618e2b5804b531c7644b5e",
              "tr_count": 16,
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0"
            },
            "cursor": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0"
          },
          {
            "node": {
              "id": "2be70019c807dfebcdee35b1972a59d30b5176097d0c20040c4a0303a7148140",
              "tr_count": 14,
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c1"
            },
            "cursor": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c1"
          },
          {
            "node": {
              "id": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c2",
              "tr_count": 18,
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c2"
            },
            "cursor": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c2"
          }
        ],
        "pageInfo": {
          "startCursor": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0",
          "endCursor": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c2",
          "hasNextPage": true
        }
      }
    }
  }
}
```

You can see cursor field in each edge object is same as cursor (because chain\_order plays the role of cursor), which can be passed over to the next query for pagination. Get the latest cursor for the page in `PageInfo.endCursor` field.

### Query all transactions/calls/internal transfers/events within block

{% hint style="info" %}
**Calls** in Acki Nacki are incoming messages attached to the transaction in transaction.in\_message object with msg\_type=0

**Internal transfers** are incoming(in\_message)  and outgoing messages (out\_messages) of the transaction with msg\_type=1

**Events** are outgoing messages (out\_messages) of transaction with msg\_type=2
{% endhint %}

1. Get the required block's chain\_order. For example chain\_order = "`7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0`"
2. Calculate chain\_order for the block with height = height+1. You can write your own code or use this JS script as an example - that parses chain\_order, adds 1 to height and encodes it back.&#x20;

```js
function isHexChar(ch) {
  return (
    (ch >= "0" && ch <= "9") ||
    (ch >= "a" && ch <= "f") ||
    (ch >= "A" && ch <= "F")
  );
}

function readDecimalPrefix(s, pos) {
  if (pos >= s.length || s[pos] < "0" || s[pos] > "9") {
    throw new Error(`Expected decimal prefix at position ${pos}`);
  }
  let end = pos;
  while (end < s.length && s[end] >= "0" && s[end] <= "9") end++;
  const prefixStr = s.slice(pos, end);
  const valueLen = Number(prefixStr) + 1; // hex chars
  return { valueLen, nextPos: end };
}

function readHexPayload(s, pos, len) {
  const end = pos + len;
  if (end > s.length) throw new Error(`Payload overruns string at position ${pos}`);
  const payload = s.slice(pos, end);
  for (let i = 0; i < payload.length; i++) {
    if (!isHexChar(payload[i])) {
      throw new Error(`Non-hex char at position ${pos + i}`);
    }
  }
  return { payload: payload.toLowerCase(), nextPos: end };
}

function parseChainOrder(chainOrder) {
  const s = chainOrder.trim();
  let pos = 0;

  const a = readDecimalPrefix(s, pos);
  const ts = readHexPayload(s, a.nextPos, a.valueLen);
  pos = ts.nextPos;

  const b = readDecimalPrefix(s, pos);
  const ph = readHexPayload(s, b.nextPos, b.valueLen);
  pos = ph.nextPos;

  const c = readDecimalPrefix(s, pos);
  const tid = readHexPayload(s, c.nextPos, c.valueLen);
  pos = tid.nextPos;

  const d = readDecimalPrefix(s, pos);
  const ht = readHexPayload(s, d.nextPos, d.valueLen);
  pos = ht.nextPos;

  if (pos !== s.length) throw new Error(`Trailing data after ${pos}`);

  return {
    timestampHex: ts.payload,
    placeholderHex: ph.payload,
    threadIdHex: tid.payload,
    threadHeightHex: ht.payload,
  };
}

function encodeField(hexPayload) {
  const v = hexPayload.toLowerCase().replace(/^0x/, "");
  return String(v.length - 1) + v;
}

function incrementThreadHeight(chainOrder) {
  const p = parseChainOrder(chainOrder);

  const oldHeight = BigInt("0x" + p.threadHeightHex);
  const newHeightHex = (oldHeight + 1n).toString(16); // no pad

  return (
    encodeField(p.timestampHex) +
    encodeField(p.placeholderHex) +
    encodeField(p.threadIdHex) +
    encodeField(newHeightHex)
  );
}

/* === Your example === */
const example =
  "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0";

console.log(incrementThreadHeight(example));
// -> 7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c1
```

3. Paginate all transactions for the block with chain\_order = "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0"

```graphql
query{
  blockchain{
    transactions(after:"7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c0", before:"7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c1" first:5){
      edges{
        node{
          now
          block_id
          id
          chain_order
          in_message{
            src
            value
            msg_type
          }
          out_messages{
            dst
            value
            msg_type
          }
          total_fees
          aborted
        }
      }
      pageInfo{
        startCursor
        endCursor
      }
    }
  }
}
```

The result:

```
{
  "data": {
    "blockchain": {
      "transactions": {
        "edges": [
          {
            "node": {
              "now": 1770181201,
              "block_id": "8f600f3cf187f89d4f1d89398edaa81a8666881aaaa02a0dd0c9654f52c1a8f6",
              "id": "0b340f91708e711e616da2d117a724e8650208b1c346feff85117dec7e25f86a",
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c001",
              "in_message": {
                "src": "",
                "value": null,
                "msg_type": 1
              },
              "out_messages": [],
              "total_fees": "0x9a28d8",
              "aborted": false
            }
          },
          {
            "node": {
              "now": 1770181201,
              "block_id": "8f600f3cf187f89d4f1d89398edaa81a8666881aaaa02a0dd0c9654f52c1a8f6",
              "id": "6a14a13a0170ddccd03b5dd889b4c9a20f33442ac35a37d83dc672e44029f905",
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c002",
              "in_message": {
                "src": "",
                "value": null,
                "msg_type": 1
              },
              "out_messages": [],
              "total_fees": "0x98a238",
              "aborted": false
            }
          },
          {
            "node": {
              "now": 1770181201,
              "block_id": "8f600f3cf187f89d4f1d89398edaa81a8666881aaaa02a0dd0c9654f52c1a8f6",
              "id": "188c575a3665d9831147c61b924665466b44d533768d01e5e35b0bb6791d28f5",
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c003",
              "in_message": {
                "src": "0:27da272e789e604aaff3cc4f23c85319366e9c02592b285609b18dd5144cd2bd",
                "value": "0x5f5e100",
                 "msg_type": 0
              },
              "out_messages": [
                {
                  "dst": "0:27da272e789e604aaff3cc4f23c85319366e9c02592b285609b18dd5144cd2bd",
                  "value": "0x5f5e100",
                   "msg_type": 0
                }
              ],
              "total_fees": "0x62eec3",
              "aborted": false
            }
          },
          {
            "node": {
              "now": 1770181201,
              "block_id": "8f600f3cf187f89d4f1d89398edaa81a8666881aaaa02a0dd0c9654f52c1a8f6",
              "id": "a20cff059c4e2915ae232c46af8164136efcd8cae2b041598542a78d7ced355c",
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c004",
              "in_message": {
                "src": "0:c91685cb50b74ae16e4c1fb896fc5e924da72bd7136ecae7a62fb0683128abec",
                "value": "0x5f5e100"
              },
              "out_messages": [
                {
                  "dst": "0:7e87112fc59c0e83812c411e714cc3f0f989c786b8f24126be275625775efc18",
                  "value": "0x5f5e100",
                  "msg_type": 0
                }
              ],
              "total_fees": "0x216568",
              "aborted": false
            }
          },
          {
            "node": {
              "now": 1770181201,
              "block_id": "8f600f3cf187f89d4f1d89398edaa81a8666881aaaa02a0dd0c9654f52c1a8f6",
              "id": "3e7e2c92ea8bb3994dd35927290aec35978c6b4d996c8cfe93bf1ebf97258194",
              "chain_order": "7698320d000670000000000000000000000000000000000000000000000000000000000000000000061d4b1c005",
              "in_message": {
                "src": "0:27da272e789e604aaff3cc4f23c85319366e9c02592b285609b18dd5144cd2bd",
                "value": "0x5f5e100",
                "msg_type": 0
              },
              "out_messages": [],
              "total_fees": "0x26ba100",
              "aborted": false
            }
          }
        ],
        "pageInfo": {
          "startCursor": "76982d251000161d3c69201",
          "endCursor": "76982d251000161d3c69205"
        }
      }
    }
  }
}
```
