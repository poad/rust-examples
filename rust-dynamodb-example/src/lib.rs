#[macro_use]
extern crate log;


pub mod dynamodb {
    use std::collections::HashMap;
    use std::io::Error;

    use rusoto_core::Region;
    use rusoto_dynamodb::{AttributeValue, CreateTableInput, CreateTableOutput, DescribeTableInput, DynamoDb, DynamoDbClient, GetItemInput, KeySchemaElement, PutItemInput, AttributeDefinition, ProvisionedThroughput, DeleteItemInput, ScanInput};
    use serde::{Deserialize, Serialize};

    use async_trait::async_trait;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Comment {
        pub(crate) name: String,
        pub(crate) message: String,
    }

    pub trait CommentAccessor {
        fn name(&self) -> String;
        fn message(&self) -> String;
    }

    impl CommentAccessor for Comment {
        fn name(&self) -> String {
            (&self.name).parse().unwrap()
        }

        fn message(&self) -> String {
            (&self.message).parse().unwrap()
        }
    }

    pub struct Client {
        endpoint: String,
        region: String,
        table: String,
    }

    #[async_trait]
    pub trait DynamoDBClient: Clone {
        fn new(endpoint: String, region: String, table: String) -> Client;

        async fn find_table(&self) -> Result<bool, Error>;
        async fn create_table(&self) -> Result<CreateTableOutput, Error>;
        async fn put_item(&self, name: String, message: String) -> Result<String, Error>;
        async fn get_item(&self, name: String) -> Result<Option<Comment>, Error>;
        async fn list_items(&self) -> Result<Vec<Comment>, Error>;
        async fn delete_item(&self, name: String) -> Result<bool, Error>;
    }

    impl Clone for Client {
        fn clone(&self) -> Self {
            Self::new(
                (&self.endpoint).parse().unwrap(),
                (&self.region).parse().unwrap(),
                (&self.table).parse().unwrap()
            )
        }
    }

    #[async_trait]
    impl DynamoDBClient for Client {
        fn new(endpoint: String, region: String, table: String) -> Client {
            Client {
                endpoint,
                region,
                table,
            }
        }

        async fn find_table(&self) -> Result<bool, Error> {
            let region = Region::Custom {
                name: (&self.region).parse().unwrap(),
                endpoint: (&self.endpoint).parse().unwrap(),
            };

            let describe_table = DescribeTableInput {
                table_name: String::from(&self.table),
                ..Default::default()
            };
            let client = DynamoDbClient::new(region);
            match client.describe_table(describe_table).await {
                Ok(result) => {
                    Ok(result.table.is_some())
                },
                Err(error) => {
                    warn!("{:?}", error);
                    Ok(false)
                }
            }
        }

        async fn create_table(&self) -> Result<CreateTableOutput, Error> {
            let region = Region::Custom {
                name: (&self.region).parse().unwrap(),
                endpoint: (&self.endpoint).parse().unwrap(),
            };

            let mut attrs: Vec<AttributeDefinition> = Vec::new();
            attrs.push(AttributeDefinition {
                attribute_name: "name".parse().unwrap(),
                attribute_type: "S".to_string()
            });

            let mut keys: Vec<KeySchemaElement> = Vec::new();
            keys.push(KeySchemaElement {
                attribute_name: "name".parse().unwrap(),
                key_type: "HASH".parse().unwrap(),
                ..Default::default()
            });

            let create_table = CreateTableInput {
                attribute_definitions: attrs,
                billing_mode: None,
                key_schema: keys,
                table_name: String::from(&self.table),
                global_secondary_indexes: None,
                local_secondary_indexes: None,
                provisioned_throughput: Some( ProvisionedThroughput {
                    read_capacity_units: 1,
                    write_capacity_units: 1
                }),
                ..Default::default()
            };
            let client = DynamoDbClient::new(region);
            match client.create_table(create_table).await {
                Ok(result) => {
                    Ok(result)
                },
                Err(error) => {
                    error!("{:?}", error);
                    panic!(error)
                }
            }
        }

        async fn put_item(&self, name: String, message: String) -> Result<String, Error> {
            let region = Region::Custom {
                name: (&self.region).parse().unwrap(),
                endpoint: (&self.endpoint).parse().unwrap(),
            };
            let table = &self.table;

            let mut item: HashMap<String, AttributeValue> = HashMap::new();
            item.insert("name".parse().unwrap(), AttributeValue {
                s: Some((&name).parse().unwrap()),
                ..Default::default()
            });
            item.insert("message".parse().unwrap(), AttributeValue {
                s: Some((&message).parse().unwrap()),
                ..Default::default()
            });



            let create_serials = PutItemInput {
                item,
                table_name: String::from(table),
                ..Default::default()
            };
            let client = DynamoDbClient::new(region);
            match client.put_item(create_serials).await {
                Ok(_result) => {
                    Ok((&name).parse().unwrap())
                },
                Err(error) => {
                    error!("{:?}", error);
                    panic!(error)
                },
            }
        }

        async fn get_item(&self, name: String) -> Result<Option<Comment>, Error> {
            let region = Region::Custom {
                name: (&self.region).parse().unwrap(),
                endpoint: (&self.endpoint).parse().unwrap(),
            };
            let table = &self.table;

            let mut key: HashMap<String, AttributeValue> = HashMap::new();
            key.insert(String::from("name"), AttributeValue {
                s: Some(name),
                ..Default::default()
            });

            let condition = GetItemInput {
                key,
                table_name: String::from(table),
                ..Default::default()
            };
            let client = DynamoDbClient::new(region);
            match client.get_item(condition).await {
                Ok(item_output) => {
                    match &item_output.item {
                        Some(item) =>
                            {
                               let comment =  (&item).get::<String>(&"name".to_string())
                                    .filter(|value| value.s.is_some())
                                    .map(|name| {
                                        item.get("message")
                                            .filter(|value| value.s.is_some())
                                            .map(|message|
                                                Comment {
                                                    name: name.s.as_ref().unwrap().parse().unwrap(),
                                                    message: message.s.as_ref().unwrap().parse().unwrap(),
                                                })
                                    });
                                comment.map_or( Ok(None), |v| Ok(v))
                            },
                        _ => Ok(Option::None)
                    }
                },
                Err(error) => {
                    error!("{:?}", error);
                    panic!(error)
                },
            }
        }

        async fn list_items(&self) -> Result<Vec<Comment>, Error> {
            let region = Region::Custom {
                name: (&self.region).parse().unwrap(),
                endpoint: (&self.endpoint).parse().unwrap(),
            };
            let table = &self.table;

            let scan = ScanInput {
                attributes_to_get: None,
                conditional_operator: None,
                consistent_read: None,
                exclusive_start_key: None,
                expression_attribute_names: None,
                expression_attribute_values: None,
                filter_expression: None,
                index_name: None,
                limit: None,
                projection_expression: None,
                return_consumed_capacity: None,
                scan_filter: None,
                segment: None,
                select: None,
                table_name: table.parse().unwrap(),
                total_segments: None
            };

            let client = DynamoDbClient::new(region);
            match client.scan(scan).await {
                Ok(output) => {
                    match output.items {
                        Some(items) => {
                            let i = Ok(items.iter().map(|item| {
                                match item.get::<String>(&"name".to_string())
                                    .filter(|value| value.s.is_some())
                                    .map( |value| value.s.as_ref().unwrap()) {
                                    Some(name) => {
                                        let v = item.get::<String>(&"message".to_string())
                                            .filter(|value| value.s.is_some())
                                            .map(|value| {
                                                let message = value.s.as_ref().unwrap();
                                                Comment {
                                                    name: name.parse().unwrap(),
                                                    message: message.parse().unwrap(),
                                                }
                                            });
                                        v
                                    },
                                    _ => None
                                }
                            })
                                .filter(|v| v.is_some())
                                .map(|v| v.unwrap())
                                .collect());
                            i
                        },
                        _ => Ok(Vec::new())
                    }
                },
                Err(error) => {
                    error!("{:?}", error);
                    panic!(error)
                },
            }
        }

        async fn delete_item(&self, name: String) -> Result<bool, Error> {
            let region = Region::Custom {
                name: (&self.region).parse().unwrap(),
                endpoint: (&self.endpoint).parse().unwrap(),
            };
            let table = &self.table;

            let mut key: HashMap<String, AttributeValue> = HashMap::new();
            key.insert(String::from("name"), AttributeValue {
                s: Some(name),
                ..Default::default()
            });

            let input = DeleteItemInput {
                condition_expression: None,
                conditional_operator: None,
                expected: None,
                expression_attribute_names: None,
                expression_attribute_values: None,
                key,
                return_consumed_capacity: None,
                return_item_collection_metrics: None,
                return_values: None,
                table_name: table.parse().unwrap()
            };

            let client = DynamoDbClient::new(region);
            match client.delete_item(input).await {
                Ok(_) => Ok(true),
                _ => Ok(false)
            }
        }
    }
}

pub mod state {
    use crate::dynamodb::Client;

    pub struct DynamoClientState {
        dynamodb: Client,
    }

    pub trait State {
        fn new(dynamodb: Client) -> DynamoClientState;
        fn client(&self) -> &Client;
    }

    impl State for DynamoClientState {
        fn new(dynamodb: Client) -> DynamoClientState {
            DynamoClientState {
                dynamodb
            }
        }

        fn client(&self) -> &Client {
            &self.dynamodb
        }
    }
}
