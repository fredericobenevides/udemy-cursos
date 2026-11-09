fun main() {
    var chef1 = Chef<Apple>()
    chef1.cook(Apple())

    var chef2 = Chef<Banana>()
    chef2.cook(Banana())
}

abstract class Fruit {
    abstract fun peel()
}

class Apple: Fruit() {
    override fun peel() {
        println("Peeling the apple")
    }
}

class Banana: Fruit() {
    override fun peel() {
        println("Peeling the banana")
    }
}

class Chef<T: Fruit> {
    fun cook(item: T) {
        item.peel()
    }
}