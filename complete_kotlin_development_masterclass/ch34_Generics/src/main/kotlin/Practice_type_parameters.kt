fun main() {
    val printer1 = Printer<Set<String>>()
    printer1.iterate(hashSetOf("Alice", "Bob", "Carol"))

    val printer2 = Printer<Set<String>>()
    printer2.iterate(hashSetOf("Dan", "Ellen", "Fran", "George"))
}

class Printer<T: Collection<String>> {
    fun iterate(collection: T) {
        collection.forEach { println(it.toString()) }
    }
}