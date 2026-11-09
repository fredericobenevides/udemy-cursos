fun main() {
    val clients = listOf("Anna", "Bob", "Carol", "Dan")

//    clients.forEach { client -> println("Hello $client") }
//    clients.forEach { println("Hello $it") }


//    clients.filter { it.length < 5 }
//        .forEach { println("Hello $it") }

//    val sizes = clients.map { it.length }
//    println(sizes)

//    val sorted = clients.sortedBy { it.length }
//    println(sorted)

//    val max = clients.maxBy { it.length }
//    println(max)

    val min = clients.minBy { it.length }
    println(min)
}