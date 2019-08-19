#!/bin/bash

list_topic() {
  kafka-topics.sh --zookeeper localhost:2181 --list
}

create_topic() {
  kafka-topics.sh --zookeeper localhost:2181 --create --topic twitter_tweets --partitions 6 --replication-factor 1

  kafka-topics.sh --zookeeper localhost:2181 --create --topic twitter_status_connect --partitions 3 --replication-factor 1
  kafka-topics.sh --zookeeper localhost:2181 --create --topic twitter_delete_connect --partitions 3 --replication-factor 1

  kafka-topics.sh --zookeeper localhost:2181 --create --topic important_tweets --partitions 3 --replication-factor 1
}

create_consumer() {
  kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic twitter_tweets

  kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic twitter_status_connect --from-beginning

  kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic important_tweets --from-beginning
}

run_elasticsearch() {
  docker run -d --name elasticsearch -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" elasticsearch:6.8.2
}

consumer_group() {
  kafka-consumer-groups.sh --bootstrap-server localhost:9092 --group kakfa-demo-elasticsearch --describe
}