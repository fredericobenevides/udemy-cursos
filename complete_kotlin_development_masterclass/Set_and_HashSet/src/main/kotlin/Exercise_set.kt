fun main() {
    val customers = hashSetOf("Michael", "Alice", "John")
    println("Input name of joining customer")
    val customer = readLine()?:""
    customers.add(customer)
    println(customer)

    println("Input name of leaving customer")
    val leavingCustomer = readLine()?:""
    customers.remove(leavingCustomer)
    println(customers)
}