# Block and Transaction Pagination: Best Practice

GraphQL Blockchain API was developed to provide a reliable way of blockchain data pagination and prevent any potential data inconsistencies regardless of network load.

Node's Blockchain API provides blocks, transactions and messages only starting from the timestamp the Node synced from.

**Note**: Query Collections are a supported instrument and will remain so. However, they are intended and optimized primarily for tasks that are not critically dependent on data completeness, such as analytics and may be turned off by your provider as these kind of queries maybe have high costs. Though you can use them on yout dedicated self-hosted node.
