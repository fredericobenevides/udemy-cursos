fun main() {
    val a1: Short = 6
    val b1: Int = 32
    var result1 = a1 * b1
    println(result1::class.java)

    val a2: Long = 2864582635
    val b2: Float = 13.67F
    val result2 = a2 / b2
    println(result2::class.java)

    val a3: Float = 234.45F
    val b3 = 34586.453
    val result3 = a3 * b3
    println(result3::class.java)
}