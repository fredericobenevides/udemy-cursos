fun main() {
    val clients = arrayListOf<String?>()
    do {
        println("Enter client anem")
        val input = readLine()
        input?.let {
            if (it.lowercase() != "stop") {
                if (it == "")
                    clients.add(null)
                else
                    clients.add(it)
            }
        }
    } while (input?.lowercase() != "stop")
    println(clients)

    clients.forEach {
        it?.let {
            println("Hello $it")
        }
    }
}