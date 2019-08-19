package com.github.simplesteph.kafka.tutorial1;

import java.util.Properties;
import java.util.concurrent.ExecutionException;

import org.apache.kafka.clients.producer.Callback;
import org.apache.kafka.clients.producer.KafkaProducer;
import org.apache.kafka.clients.producer.ProducerConfig;
import org.apache.kafka.clients.producer.ProducerRecord;
import org.apache.kafka.clients.producer.RecordMetadata;
import org.apache.kafka.common.serialization.StringSerializer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ProducerDemoKeys {
    private static final String bootstrapServers = "127.0.0.1:9092";

    private static Logger logger = LoggerFactory.getLogger(ProducerDemoKeys.class);

    public static void main(String[] args) throws InterruptedException, ExecutionException {
        // create Producer properties
        Properties properties = new Properties();
        properties.setProperty(ProducerConfig.BOOTSTRAP_SERVERS_CONFIG, bootstrapServers);
        properties.setProperty(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, StringSerializer.class.getName());
        properties.setProperty(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, StringSerializer.class.getName());

        // create the producer
        KafkaProducer<String, String> producer = new KafkaProducer<>(properties);

        for (int i = 0; i < 10; i++) {            
            // create a produer record
            String topic = "first_topic";
            String value = "hello world " + String.valueOf(i);
            String key = "id_" + String.valueOf(i);


            ProducerRecord<String, String> record = new ProducerRecord<>(topic, key, value);

            logger.info("********* Key: " + key);
    
            // send data - asynchronous
            producer.send(record, new Callback(){
            
                @Override
                public void onCompletion(RecordMetadata metadata, Exception e) {
                    // executes every time a record is successfully sent or an exception is thrown
                    if (e == null) {
                        // the record was successfully sent
                        logger.info("Received new metadata. \n" +
                        "Topic:" + metadata.topic() + "\n" +
                        "Partition:" + metadata.offset() + "\n" +
                        "Timestamp: " + metadata.timestamp());
                        
                    } else {
                        logger.error("Error while producing", e);
                    }
                }
            }).get(); // block the .send() to make it synchronous - don`t do this in production!
        }

        // flush
        producer.flush();

        // flush and close
        producer.close();
    }
}