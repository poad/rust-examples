
```
docker-compose up -d dynamodb
docker run --rm -it --net rust-dynamodb-example_dynamodb-network \
 -e AWS_ACCESS_KEY_ID=test \
 -e AWS_SECRET_ACCESS_KEY=test \
 -e AWS_DEFAULT_REGION=us-west-2 \
 --entrypoint bash amazon/aws-cli

aws --endpoint-url http://dynamodb:8000 \
    dynamodb create-table --table-name test \
        --key-schema AttributeName=name,KeyType=HASH \
        --attribute-definitions AttributeName=name,AttributeType=S \
        --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5
exit

docker-compose up -d
```