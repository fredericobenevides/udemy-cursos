fun main() {
    val myBox = Box<String>()
    myBox.display("Contents")

    val carBox = Box<Car>()
    carBox.display(Car())

    val newBox = NewBox<Int, Float>()
    newBox.display(34, 57.4F)
}

class Box<T> {
    fun display(item: T) {
        println(item)
    }
}

class NewBox<T, U> {
    fun display(item: T, item2: U) {
        println(item)
        println(item2)
    }
}

class Car {

}