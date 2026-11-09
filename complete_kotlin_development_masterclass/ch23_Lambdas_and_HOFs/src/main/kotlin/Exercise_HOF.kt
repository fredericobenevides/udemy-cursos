fun main() {
    val clients = arrayListOf("Michael", "Jane", "Dan", "Michelle")
    val messages = getMessages(clients) { name -> "Hello $name how are you?" }
    println(clients)
    println(messages)
}

fun getMessages(clients: Collection<String>, getMessage: (String) -> String): ArrayList<String> {
    val messages = arrayListOf<String>()
    for (client in clients) {
        messages.add(getMessage(client))
    }
    return messages
}