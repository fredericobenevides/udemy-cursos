import kotlin.random.Random

fun main() {
    val someLargeVariable: String by lazy {
        "Some large value"
    }

    if (Random.nextInt() % 2 == 0)
        println(someLargeVariable)
}


