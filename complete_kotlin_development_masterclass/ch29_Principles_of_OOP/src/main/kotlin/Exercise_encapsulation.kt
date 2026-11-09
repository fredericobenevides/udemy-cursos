fun main() {
    val myRestaurant = Franchise()
    myRestaurant.prepareBurgers()
}

open class OriginalRestaurant {
    protected fun applyFormula(): String {
        return "Prepare burgers with love and care"
    }
}

class Franchise: OriginalRestaurant() {
    fun prepareBurgers() {
        println("Preparing burgers according to the secret formula")
        println(applyFormula())
    }
}