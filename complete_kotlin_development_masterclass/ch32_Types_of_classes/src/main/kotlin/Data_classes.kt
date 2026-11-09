fun main() {
    val myUser = User("John", "john@gmail.com", "johnIsCool")
    println(myUser.toString())

    val myUser2 = User("John", "john@gmail.com", "johnIsCool")
    println(myUser == myUser2)

    val myUser3 = myUser2.copy(email = "john1@gmail.com")
    println(myUser3)
}

data class User(
    val name: String,
    val email: String,
    val password: String
)