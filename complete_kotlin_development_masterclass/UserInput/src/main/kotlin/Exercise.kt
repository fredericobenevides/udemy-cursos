fun main() {
    println("Please enter your birth year")
    val yearInput = readLine()?:""
    val year = yearInput.toInt()
    val age = 2025 - year;
    println("Your age is either ${age - 1} or $age")
}