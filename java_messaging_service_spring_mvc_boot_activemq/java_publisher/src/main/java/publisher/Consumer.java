package publisher;

import org.apache.activemq.ActiveMQConnectionFactory;

import javax.jms.*;

public class Consumer {

    public static void main(String[] args) {
        ConnectionFactory connectionFactory = new ActiveMQConnectionFactory("admin", "admin",
                "tcp://localhost:61616" );
        try {
            Connection connection = connectionFactory.createConnection();
            connection.start();

            Session session = connection.createSession(false, Session.CLIENT_ACKNOWLEDGE);
            Destination destination = session.createQueue("demo");

            MessageConsumer consumer = session.createConsumer(destination);
            consumer.setMessageListener((message) -> {
               TextMessage textMessage = (TextMessage) message;
                try {
                    System.out.println(textMessage.getText());
                    textMessage.acknowledge();
                } catch (JMSException e) {
                    e.printStackTrace();
                }
            });

        } catch (JMSException e) {
            e.printStackTrace();
        }
    }
}
