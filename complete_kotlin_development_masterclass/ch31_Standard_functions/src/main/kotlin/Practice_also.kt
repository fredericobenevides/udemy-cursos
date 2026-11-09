fun main() {
    Book().apply {
        printBook()
    }
        .also {
            println("LOG: printing book $it")
            println("Sending an email about book $it")
        }
}

class Book {
    fun printBook() {
        println("Printing a book")
    }
}