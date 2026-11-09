fun main() {
    val cats = 3
    println(cats::class.java)
    val catsString = cats.toString()
    println(catsString::class.java)

    val pi = 3.1415
    println(pi::class.java)
    val piString = pi.toString()
    println(piString::class.java)
}