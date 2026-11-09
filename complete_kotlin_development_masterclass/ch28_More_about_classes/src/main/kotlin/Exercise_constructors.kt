fun main() {
    val user1 = UserAccount()
    val user2 = UserAccount("John")
    val user3 = UserAccount("Mary", 175)
    val user4 = UserAccount("Anna", 2635)
}

class UserAccount {
    constructor() {
        userName = "Generic user"
        balance = 1
        canAfford()
    }

    constructor(newUserName: String) {
        userName = newUserName
        balance = 2
        canAfford()
    }

    constructor(newUserName: String, newBalance: Int) {
        userName = newUserName
        balance = newBalance
        canAfford()
    }

    var userName = "User name"
    var balance = 0
    val tshirtPrice = 20

    fun canAfford() {
        if (balance > tshirtPrice) {
            println("$userName can afford ${balance / tshirtPrice} tshirts with the balance $balance")
        } else {
            println("$userName cannot afford any tshirts with the balance $balance")
        }
    }
}