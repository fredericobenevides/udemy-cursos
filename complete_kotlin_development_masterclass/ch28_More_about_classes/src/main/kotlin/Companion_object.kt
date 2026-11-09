fun main() {
//    println(Car4().provideDrivingInstructions())
    println(Car4.provideDrivingInstructions())
}

class Car4 {
    companion object {
        fun provideDrivingInstructions(): String {
            return "Drive safe!"
        }
    }
}