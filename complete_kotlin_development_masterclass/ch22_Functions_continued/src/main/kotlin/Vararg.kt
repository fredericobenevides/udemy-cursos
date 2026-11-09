fun main() {
    sayHello("Alice", "Bob", "Carol", "Dan", "Fiona", "Eric")
}

fun sayHello(vararg names: String) {
    for (name in names) {
        println("Hello $name")
    }
}