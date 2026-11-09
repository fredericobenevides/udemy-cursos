import kotlin.random.Random

fun main() {
    for (i in 1..10) {
        println("$i.")
        val service = getNetworkService()
        (service as? FTPService)?.transferFile()
    }
}

open class NetworkService {}
class FTPService: NetworkService() {
    fun transferFile() {
        println("Transferring a file via FTP")
    }
}

fun getNetworkService(): NetworkService {
    return if (Random.nextInt() % 2 == 0)
        NetworkService()
    else
        FTPService()
}