fun main() {
    val note = PostItNote()
//    println(note.message)
    note.printMessage()
    note.updateMessage("Meeting at 5")
    //    println(note.message)
    note.printMessage()
}

class PostItNote() {
    var message: String = "No message"

    fun updateMessage(message: String) {
        this.message = message
    }

    fun printMessage() {
        println(this.message)
    }
}