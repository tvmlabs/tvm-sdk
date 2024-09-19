# Blocks

## Get the block info

### By seq\_no

Specify the workchain\_id, thread (shard) and seq\_no:

```graphql
query{
  blockchain{
    block_by_seq_no(workchain:0, shard:"1800000000000000", seq_no:22723155){
      id
      hash
      seq_no
    }
  }
}
```

Result:

```graphql
{
  "data": {
    "blockchain": {
      "block_by_seq_no": {
        "id": "block/8f1fa341f9d419dea23c621c659fe4534d7d4f3aab2abdf512eeb90da919a956",
        "hash": "8f1fa341f9d419dea23c621c659fe4534d7d4f3aab2abdf512eeb90da919a956",
        "seq_no": 22723155
      }
    }
  }
}
```

### By hash

Specify the block hash:

```graphql
query{
  blockchain{
    block(hash:"8f1fa341f9d419dea23c621c659fe4534d7d4f3aab2abdf512eeb90da919a956"){
      id
      hash
      seq_no
    }
  }
}
```

Result:

```graphql
{
  "data": {
    "blockchain": {
      "block": {
        "id": "block/8f1fa341f9d419dea23c621c659fe4534d7d4f3aab2abdf512eeb90da919a956",
        "hash": "8f1fa341f9d419dea23c621c659fe4534d7d4f3aab2abdf512eeb90da919a956",
        "seq_no": 22723155
      }
    }
  }
}
```

## Blocks pagination

### About cursor

This cursor splits all the blockchain threads' blocks into ranges between timestamps.

### Paginate by blocks timestamp range <a href="#paginate_by_seqno" id="paginate_by_seqno"></a>

We specify timestamp range. If shard parameter is omitted - you will get all shards blocks.

```graphql
query {
  blockchain{
    blocks(
      master_seq_no_range: {
        start: 1725442336 # start timestamp
        end: 1725528736 # end timestamp
      }
    ) {
      edges {
        node {
          id
          shard
          seq_no
          hash
          file_hash
        }
            cursor
      }
          pageInfo{
            endCursor
          }
    }
  }
}
```

Result:

You can see cursor field in each edge object, which can be passed over to the next query for pagination. Or you can get the latest cursor for the result set in `PageInfo.endCursor` field.

```graphql
{
  "data": {
    "blockchain": {
      "blocks": {
        "edges": [
          {
            "node": {
              "id": "block/279dd285a1a73dfcdd0b3739751232be6d6669821e53dd1b450c67e8f59d651d",
              "shard": "8000000000000000",
              "seq_no": 2773451,
              "hash": "279dd285a1a73dfcdd0b3739751232be6d6669821e53dd1b450c67e8f59d651d",
              "file_hash": "f7d7f2a506413c736eb3cb6ca3b071c15dc953f6d5e1996244fca68a2594fecf"
            },
            "cursor": "52899360052a51cb01"
          },
          {
            "node": {
              "id": "block/b146ba0ce2fd140b18f42199de57128d2a4de65e38676ca007b68ff0ab4f4b60",
              "shard": "8000000000000000",
              "seq_no": 2773452,
              "hash": "b146ba0ce2fd140b18f42199de57128d2a4de65e38676ca007b68ff0ab4f4b60",
              "file_hash": "c35b121b5edfa3389e64fb44d937553e6b3928877b14505559a168670953055c"
            },
            "cursor": "52899370052a51cc01"
          },
        ...
        ],
        "pageInfo": {
          "endCursor": "52899800052a51fc01"
        }
      }
    }
  }
}
```

Next page:

Let's check other available parameters for pagination.

`after/first` - Show `first` number of items `after` (not including) cursor.

`before/last`- Show `last` number of items `before` (not including) cursor. Used for backward pagination.

To check if the next page exists - we ask for `pageInfo.hasNextPage` parameter. If no next page exists, we can move `seq_no` range forward. If you implement backward pagination - use `pageInfo.hasPreviousPage.`

Check other available parameters in GraphQL schema in playground.

Here we continue pagination within the same `seq_no` range, and ask for the next 10 blocks after the last cursor in the previous query. We see that the next page exists so we can continue paginating within the same `seq_no` range.

```graphql
query {
    blockchain {
        blocks(
            master_seq_no_range: {
                start: 1725442336 # start timestamp
                end: 1725528736   # end timestamp
            }
            after: "52899800052a51fc01"
            first: 10
        ) {
            edges {
                node {
                    id
                    shard
                    seq_no
                    hash
                    file_hash
                }
                cursor
            }
            pageInfo {
                endCursor
                hasNextPage
            }
        }
    }
}
```

Result:

```graphql
{
  "data": {
    "blockchain": {
      "blocks": {
        "edges": [
          {
            "node": {
              "id": "block/279dd285a1a73dfcdd0b3739751232be6d6669821e53dd1b450c67e8f59d651d",
              "shard": "8000000000000000",
              "seq_no": 2773451,
              "hash": "279dd285a1a73dfcdd0b3739751232be6d6669821e53dd1b450c67e8f59d651d",
              "file_hash": "f7d7f2a506413c736eb3cb6ca3b071c15dc953f6d5e1996244fca68a2594fecf"
            },
            "cursor": "52899360052a51cb01"
          },
          {
            "node": {
              "id": "block/b146ba0ce2fd140b18f42199de57128d2a4de65e38676ca007b68ff0ab4f4b60",
              "shard": "8000000000000000",
              "seq_no": 2773452,
              "hash": "b146ba0ce2fd140b18f42199de57128d2a4de65e38676ca007b68ff0ab4f4b60",
              "file_hash": "c35b121b5edfa3389e64fb44d937553e6b3928877b14505559a168670953055c"
            },
            "cursor": "52899370052a51cc01"
          },
      ..... other blocks
        ],
        "pageInfo": {
          "endCursor": "52899800052a51fc01"
          "hasNextPage": true
        }
      }
    }
  }
}
```

## Query the latest block height

Let's get the latest default shard 0800000000000000 block height.

This query can be used to detect the Node's health.

```graphql
query {
  blockchain{
   blocks(last:1){
          edges {
           node {
            seq_no
           }
          }
   }
  }
}

```

The block height is `1948985`:

```graphql
{
  "data": {
    "blockchain": {
      "blocks": {
        "edges": [
          {
            "node": {
              "seq_no": 2780615
            }
          }
        ]
      }
    }
  }
}
```

## How to get other blocks data

Acki Nacki Node is not a simple node with key-value storage, but a node with an embedded SQL DB, so using SQL queries you can get any other data and analytics. Use DB UI like Beaver to have manual access to this data.
