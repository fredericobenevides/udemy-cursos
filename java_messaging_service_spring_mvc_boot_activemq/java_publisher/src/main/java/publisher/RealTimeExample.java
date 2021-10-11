package publisher;

import org.apache.activemq.ActiveMQConnectionFactory;
import org.json.JSONObject;

import javax.jms.*;

public class RealTimeExample {

    public static void main(String[] args) {
        ConnectionFactory connectionFactory = new ActiveMQConnectionFactory("admin", "admin",
                "tcp://localhost:61616");
        try {
            Connection connection = connectionFactory.createConnection();
            Session session = connection.createSession(false, Session.AUTO_ACKNOWLEDGE);
            Destination destination = session.createQueue("demo");

            JSONObject json = new JSONObject();
            json.put("from_date", "01-Jan-2019");
            json.put("to_date", "31-Dec-2019");
            json.put("email", "xyz@gmail.com");
            json.put("query", "select * from data");

            TextMessage textMessage = session.createTextMessage(json.toString());

            MessageProducer producer = session.createProducer(destination);
            producer.send(textMessage);

            session.close();
            connection.close();
        } catch (JMSException e) {
            e.printStackTrace();
        }
    }
}
