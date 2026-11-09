fun main() {
//    println("What is your anem?")
//    val userName = readLine()
//    println("Welcome, $userName")

    println("Please enter a number")
    val number = readLine()?:""
    val result = number.toInt() * 5
    println("$number * 5 = $result")
}